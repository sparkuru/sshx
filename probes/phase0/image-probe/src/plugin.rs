use crate::image::{self, fail, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use sshx_phase0_protocol::{read_frame, write_frame, RequestCounter, INTEGER_MAX};
use std::io::{BufRead, Write};

struct Peer<R, W> {
    input: R,
    output: W,
    next: u64,
}
impl<R: BufRead, W: Write> Peer<R, W> {
    fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        if self.next > INTEGER_MAX {
            return Err(fail("request counter exhausted"));
        }
        let id = self.next.to_string();
        self.next += 1;
        write_frame(
            &mut self.output,
            &json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}),
        )?;
        let response = read_frame(&mut self.input)?.ok_or_else(|| fail("broker disconnected"))?;
        if response["id"] != id || response.get("error").is_some() {
            return Err(fail("broker refused image stage"));
        }
        response
            .get("result")
            .cloned()
            .ok_or_else(|| fail("invalid broker response"))
    }
    fn encode_action(&mut self, operation: &str, grants: &Value) -> Result<()> {
        let raw = self.request(
            "clipboard.read_image",
            json!({"operation_id":operation,"grant_ids":grants}),
        )?;
        let width = u32::try_from(raw["width"].as_u64().ok_or_else(|| fail("invalid width"))?)?;
        let height = u32::try_from(
            raw["height"]
                .as_u64()
                .ok_or_else(|| fail("invalid height"))?,
        )?;
        let length = image::layout(width, height)?;
        if raw["format"] != "rgba8"
            || raw["stride"] != u64::from(width) * 4
            || raw["byte_length"] != length as u64
        {
            return Err(fail("CORRUPT_IMAGE"));
        }
        let blob = raw["blob_id"]
            .as_str()
            .ok_or_else(|| fail("missing blob"))?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length)?;
        while bytes.len() < length {
            let requested = (length - bytes.len()).min(32768);
            let part = self.request("blob.read", json!({"operation_id":operation,"blob_id":blob,"offset":bytes.len(),"length":requested}))?;
            let encoded = part["data_base64"]
                .as_str()
                .ok_or_else(|| fail("invalid chunk"))?;
            if encoded.len() > 43692 || part["offset"] != bytes.len() as u64 {
                return Err(fail("LIMIT_EXCEEDED"));
            }
            let chunk = STANDARD.decode(encoded)?;
            if chunk.is_empty() || chunk.len() > requested {
                return Err(fail("CORRUPT_IMAGE"));
            }
            bytes.extend_from_slice(&chunk);
        }
        let png = image::encode(width, height, &bytes)?;
        let created = self.request(
            "blob.create",
            json!({"operation_id":operation,"kind":"png","byte_length":png.len()}),
        )?;
        let output_blob = created["blob_id"]
            .as_str()
            .ok_or_else(|| fail("missing output blob"))?;
        let mut offset = 0;
        for chunk in png.chunks(32768) {
            let answer = self.request("blob.write", json!({"operation_id":operation,"blob_id":output_blob,"offset":offset,"data_base64":STANDARD.encode(chunk)}))?;
            offset += chunk.len();
            if answer["next_offset"] != offset as u64 {
                return Err(fail("invalid blob acknowledgement"));
            }
        }
        self.request(
            "blob.seal",
            json!({"operation_id":operation,"blob_id":output_blob,"sha256":image::digest(&png)}),
        )?;
        self.request("ui.notify", json!({"operation_id":operation,"level":"info","text":"Synthetic image encoded and sealed; transfer remains a separate broker probe"}))?;
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut peer = Peer {
        input: stdin.lock(),
        output: stdout.lock(),
        next: 1,
    };
    let initialize = read_frame(&mut peer.input)?.ok_or_else(|| fail("missing initialize"))?;
    let mut incoming = RequestCounter::default();
    incoming.admit(
        initialize["id"]
            .as_str()
            .ok_or_else(|| fail("request ID required"))?,
    )?;
    if initialize["method"] != "plugin.initialize" {
        return Err(fail("initialization required"));
    }
    let versions = initialize["params"]["api_versions"]
        .as_array()
        .ok_or_else(|| fail("invalid versions"))?;
    if !versions.iter().any(|version| version == "1.0") {
        write_frame(
            &mut peer.output,
            &json!({"jsonrpc":"2.0","id":initialize["id"],"error":{"code":-32000,"message":"No common API","data":{"name":"VERSION_MISMATCH","retryable":false,"operation_id":null,"phase":null}}}),
        )?;
        return Ok(());
    }
    write_frame(
        &mut peer.output,
        &json!({"jsonrpc":"2.0","id":initialize["id"],"result":{"api_version":"1.0","plugin_version":"0.1.0","features":[]}}),
    )?;
    let ready = read_frame(&mut peer.input)?.ok_or_else(|| fail("missing ready"))?;
    if ready["method"] != "plugin.ready" {
        return Err(fail("ready required"));
    }
    peer.request(
        "action.register",
        json!({"actions":[{"id":"image","title":"Encode one authorized image"}]}),
    )?;
    while let Some(request) = read_frame(&mut peer.input)? {
        let method = request["method"]
            .as_str()
            .ok_or_else(|| fail("request required"))?;
        let id = &request["id"];
        incoming.admit(id.as_str().ok_or_else(|| fail("request ID required"))?)?;
        match method {
            "action.run" => {
                let operation = request["params"]["operation_id"]
                    .as_str()
                    .ok_or_else(|| fail("operation required"))?;
                write_frame(
                    &mut peer.output,
                    &json!({"jsonrpc":"2.0","id":id,"result":{"accepted":true}}),
                )?;
                peer.encode_action(operation, &request["params"]["grant_ids"])?;
            }
            "plugin.stop" => {
                write_frame(
                    &mut peer.output,
                    &json!({"jsonrpc":"2.0","id":id,"result":{"stopping":true}}),
                )?;
                break;
            }
            _ => write_frame(
                &mut peer.output,
                &json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Method not found"}}),
            )?,
        }
    }
    Ok(())
}
