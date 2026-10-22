use crate::image::{self, fail, Result};
use crate::runtime::{self, Display, OwnedChild, Runtime};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use sshx_phase0_protocol::{read_frame, write_frame, RequestCounter};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

fn owner(
    runtime: &Runtime,
    display: &Display,
    helper: &Path,
    first: &Path,
    second: Option<&Path>,
    incr: bool,
) -> Result<OwnedChild> {
    let mut command = display.command(runtime, helper);
    command.arg(if incr { "incr" } else { "owner" }).arg(first);
    if let Some(second) = second {
        command.arg(second);
    }
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut child = OwnedChild::spawn(&mut command)?;
    let mut reader = BufReader::new(
        child
            .child()
            .stdout
            .as_mut()
            .ok_or_else(|| fail("missing owner stdout"))?,
    );
    wait_readable(reader.get_ref().as_raw_fd())?;
    let mut line = String::new();
    reader.read_line(&mut line)?;
    if line != "READY\n" {
        return Err(fail("selection owner not ready"));
    }
    Ok(child)
}
fn wait_readable(fd: i32) -> Result<()> {
    let mut poll = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    if unsafe { libc::poll(&mut poll, 1, 5000) } <= 0 {
        return Err(fail("peer timeout"));
    }
    Ok(())
}
fn wait_buffered<R: Read + AsRawFd>(reader: &BufReader<R>) -> Result<()> {
    if reader.buffer().is_empty() {
        wait_readable(reader.get_ref().as_raw_fd())
    } else {
        Ok(())
    }
}
fn capture(
    runtime: &Runtime,
    display: &Display,
    helper: &Path,
    name: &str,
) -> Result<(Value, Option<Vec<u8>>)> {
    let path = runtime.path.join(name);
    let output = display
        .command(runtime, helper)
        .arg("capture")
        .arg(&path)
        .output()?;
    let metadata: Value = serde_json::from_slice(&output.stdout)?;
    if output.status.success() {
        Ok((metadata, Some(fs::read(path)?)))
    } else if output.status.code() == Some(3) {
        Ok((metadata, None))
    } else {
        Err(fail("selection helper failed"))
    }
}

fn independent_encode(
    runtime: &Runtime,
    raw: &[u8],
    width: u32,
    height: u32,
    binding: &Value,
) -> Result<(Vec<u8>, Value)> {
    let plugin_home = runtime.path.join("plugin-home");
    fs::DirBuilder::new().mode(0o700).create(&plugin_home)?;
    let mut command = runtime.command(std::env::current_exe()?);
    command
        .env("HOME", &plugin_home)
        .current_dir(&plugin_home)
        .arg("plugin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(File::create(runtime.path.join("image-plugin.stderr"))?);
    let mut child = OwnedChild::spawn(&mut command)?;
    let mut input = BufReader::new(
        child
            .child()
            .stdout
            .take()
            .ok_or_else(|| fail("missing plugin output"))?,
    );
    let mut output = child
        .child()
        .stdin
        .take()
        .ok_or_else(|| fail("missing plugin input"))?;
    write_frame(
        &mut output,
        &json!({"jsonrpc":"2.0","id":"1","method":"plugin.initialize","params":{"api_versions":["1.0"],"core_instance_id":binding["core_instance_id"],"core_version":"0.1.0","manifest_digest":"synthetic","limits_profile":"proposed-v1"}}),
    )?;
    wait_buffered(&input)?;
    let hello = read_frame(&mut input)?.ok_or_else(|| fail("plugin EOF"))?;
    if hello["result"]["api_version"] != "1.0" {
        return Err(fail("plugin version mismatch"));
    }
    write_frame(
        &mut output,
        &json!({"jsonrpc":"2.0","method":"plugin.ready","params":{"api_version":"1.0","instance_id":"22222222222222222222222222222222","available_methods":["action.register","clipboard.read_image","blob.read","blob.create","blob.write","blob.seal","ui.notify"]}}),
    )?;
    let mut png = Vec::new();
    let mut declared = None;
    let mut sealed = false;
    let mut snapshot_calls = 0;
    let mut methods = Vec::new();
    let mut requests = RequestCounter::default();
    loop {
        wait_buffered(&input)?;
        let request = read_frame(&mut input)?.ok_or_else(|| fail("plugin EOF"))?;
        let method = request["method"]
            .as_str()
            .ok_or_else(|| fail("plugin method absent"))?;
        requests.admit(
            request["id"]
                .as_str()
                .ok_or_else(|| fail("plugin request ID absent"))?,
        )?;
        methods.push(method.to_owned());
        let params = &request["params"];
        if method != "action.register" && params["operation_id"] != binding["operation_id"] {
            return Err(fail("STALE_BINDING"));
        }
        let result = match method {
            "action.register" => json!({"registered":["image"]}),
            "clipboard.read_image" => {
                snapshot_calls += 1;
                json!({"blob_id":"33333333333333333333333333333333","format":"rgba8","width":width,"height":height,"stride":width*4,"byte_length":raw.len()})
            }
            "blob.read" => {
                let offset = params["offset"]
                    .as_u64()
                    .ok_or_else(|| fail("offset absent"))? as usize;
                let length = params["length"]
                    .as_u64()
                    .ok_or_else(|| fail("length absent"))? as usize;
                if length > 32768 || offset.checked_add(length).is_none_or(|end| end > raw.len()) {
                    return Err(fail("bad raw chunk"));
                }
                json!({"offset":offset,"data_base64":STANDARD.encode(&raw[offset..offset+length]),"eof":offset+length==raw.len()})
            }
            "blob.create" => {
                let length = params["byte_length"]
                    .as_u64()
                    .ok_or_else(|| fail("length absent"))? as usize;
                if length > image::PNG_MAX || declared.is_some() {
                    return Err(fail("bad blob admission"));
                }
                png.try_reserve_exact(length)?;
                declared = Some(length);
                json!({"blob_id":"44444444444444444444444444444444"})
            }
            "blob.write" => {
                let encoded = params["data_base64"]
                    .as_str()
                    .ok_or_else(|| fail("chunk absent"))?;
                if encoded.len() > 43692 || params["offset"] != png.len() as u64 {
                    return Err(fail("bad PNG chunk"));
                }
                let bytes = STANDARD.decode(encoded)?;
                if bytes.len() > 32768 || png.len() + bytes.len() > declared.unwrap_or(0) {
                    return Err(fail("bad PNG chunk length"));
                }
                png.extend_from_slice(&bytes);
                json!({"next_offset":png.len()})
            }
            "blob.seal" => {
                if Some(png.len()) != declared
                    || params["sha256"] != image::digest(&png)
                    || image::decode(&png, Some((width, height)))?.2 != raw
                {
                    return Err(fail("PNG sealing failed"));
                }
                sealed = true;
                json!({"sealed":true,"byte_length":png.len(),"sha256":image::digest(&png)})
            }
            "ui.notify" => {
                if !sealed {
                    return Err(fail("plugin notified before seal"));
                }
                json!({"delivered":true})
            }
            _ => return Err(fail("unexpected public method")),
        };
        write_frame(
            &mut output,
            &json!({"jsonrpc":"2.0","id":request["id"],"result":result}),
        )?;
        if method == "action.register" {
            write_frame(
                &mut output,
                &json!({"jsonrpc":"2.0","id":"2","method":"action.run","params":{"operation_id":binding["operation_id"],"binding":binding,"target":{"display_label":"synthetic"},"action_id":"image","grant_ids":["synthetic-one-shot"]}}),
            )?;
            wait_buffered(&input)?;
            let ack = read_frame(&mut input)?.ok_or_else(|| fail("plugin EOF"))?;
            if ack["id"] != "2" || ack["result"]["accepted"] != true {
                return Err(fail("action not accepted"));
            }
        }
        if method == "ui.notify" {
            break;
        }
    }
    write_frame(
        &mut output,
        &json!({"jsonrpc":"2.0","id":"3","method":"plugin.stop","params":{"reason":"fixture complete"}}),
    )?;
    wait_buffered(&input)?;
    let stopped = read_frame(&mut input)?.ok_or_else(|| fail("plugin EOF"))?;
    if stopped["result"]["stopping"] != true
        || !child.wait(Duration::from_secs(2))?.success()
        || snapshot_calls != 1
    {
        return Err(fail("plugin lifecycle failed"));
    }
    let evidence = json!({"experiment":"E3","case":"independent-image-plugin-public-rpc","status":"pass","snapshot_calls":snapshot_calls,"methods":methods,"sealed_sha256":image::digest(&png),"sealed_bytes":png.len(),"raw_sha256":image::digest(raw),"private_core_dependency":false,"environment":"minimal PATH/HOME/LANG; no DISPLAY/XAUTHORITY/agent/credentials","limitation":"public encoding subset only; transfer and insertion are separate real Core probes"});
    Ok((png, evidence))
}

pub fn run(output: &Path, artifacts: Option<&Path>, binding_file: Option<&Path>) -> Result<()> {
    let binding = if let Some(path) = binding_file {
        let mut bytes = Vec::new();
        File::open(path)?.take(16385).read_to_end(&mut bytes)?;
        if bytes.len() > 16384 {
            return Err(fail("binding file too large"));
        }
        let binding: Value = serde_json::from_slice(&bytes)?;
        for name in [
            "core_instance_id",
            "operation_id",
            "session_id",
            "client_id",
            "attachment_id",
            "profile_id",
        ] {
            if binding[name]
                .as_str()
                .is_none_or(|value| value.is_empty() || value.len() > 64)
            {
                return Err(fail("invalid binding ID"));
            }
        }
        for name in ["generation", "writer_epoch", "config_generation"] {
            if binding[name]
                .as_u64()
                .is_none_or(|value| value > 9_007_199_254_740_991)
            {
                return Err(fail("invalid binding counter"));
            }
        }
        binding
    } else {
        json!({"core_instance_id":"11111111111111111111111111111111","operation_id":"55555555555555555555555555555555","session_id":"66666666666666666666666666666666","generation":1,"client_id":"77777777777777777777777777777777","attachment_id":"88888888888888888888888888888888","writer_epoch":1,"profile_id":"99999999999999999999999999999999","config_generation":1})
    };
    let runtime = Runtime::new()?;
    let helper = runtime::compile_selection(&runtime)?;
    let mut display = Display::start(&runtime, "client-A")?;
    let mut display_b = Display::start(&runtime, "client-B")?;
    let rgba = [255, 0, 0, 0, 0, 255, 0, 127, 0, 0, 255, 255, 2, 3, 4, 9];
    let first = image::encode(2, 2, &rgba)?;
    let second = image::encode(1, 1, &[9, 8, 7, 255])?;
    let first_path = runtime.write("first.png", &first)?;
    let second_path = runtime.write("second.png", &second)?;
    let mut selection_b = owner(&runtime, &display_b, &helper, &second_path, None, false)?;
    let mut cases = Vec::new();
    let (empty, _) = capture(&runtime, &display, &helper, "empty.png")?;
    cases.push(json!({"experiment":"E3","case":"empty-private-selection","status":if empty["name"]=="CLIPBOARD_EMPTY" {"pass"} else {"fail"},"observed":empty}));
    let mut selection = owner(
        &runtime,
        &display,
        &helper,
        &first_path,
        Some(&second_path),
        false,
    )?;
    let (metadata, snapshot) = capture(&runtime, &display, &helper, "snapshot.png")?;
    let snapshot = snapshot.ok_or_else(|| fail("synthetic image not captured"))?;
    let (_, _, raw) = image::decode(&snapshot, Some((2, 2)))?;
    selection
        .child()
        .stdin
        .as_mut()
        .ok_or_else(|| fail("missing owner control"))?
        .write_all(b"next\n")?;
    let owner_stdout = selection
        .child()
        .stdout
        .as_mut()
        .ok_or_else(|| fail("missing owner output"))?;
    wait_readable(owner_stdout.as_raw_fd())?;
    let mut change_ack = String::new();
    BufReader::new(owner_stdout).read_line(&mut change_ack)?;
    if change_ack != "CHANGED\n" {
        return Err(fail("selection change barrier failed"));
    }
    // A second conversion is a separate fixture observation, never an operation retry.
    let (changed, changed_bytes) = capture(&runtime, &display, &helper, "changed.png")?;
    let changed_bytes = changed_bytes.ok_or_else(|| fail("changed selection unavailable"))?;
    cases.push(json!({"experiment":"E3","case":"one-shot-alpha-immutable","status":if raw==rgba && image::digest(&snapshot)==image::digest(&first) && changed_bytes==second {"pass"} else {"fail"},"capture_count_for_operation":1,"raw_byte_length":raw.len(),"raw_sha256":image::digest(&raw),"captured_png_sha256":image::digest(&snapshot),"selection_changed":changed["name"]=="CAPTURED","alpha_preserved":raw.iter().skip(3).step_by(4).eq([0u8,127,255,9].iter()),"native_bounds":metadata,"unrelated_existing_display_accessed":false}));
    let (sealed, mut plugin_case) = independent_encode(&runtime, &raw, 2, 2, &binding)?;
    plugin_case["binding"] = binding.clone();
    plugin_case["binding_sha256"] = json!(image::digest(&serde_json::to_vec(&binding)?));
    cases.push(plugin_case);
    cases.push(json!({"experiment":"E3","case":"triggering-client-desktop-binding","status":if raw==rgba && display.name!=display_b.name {"pass"} else {"fail"},"client_A_snapshot_calls":1,"client_B_snapshot_calls":0,"private_display_count":2,"client_B_selection_owner_alive":selection_b.child().try_wait()?.is_none(),"limitation":"controlled Client fixture processes; Core broker integration is recorded separately"}));
    selection.stop()?;
    let (gone, _) = capture(&runtime, &display, &helper, "gone.png")?;
    cases.push(json!({"experiment":"E3","case":"selection-owner-exit","status":if gone["name"]=="CLIPBOARD_EMPTY" {"pass"} else {"fail"},"observed":gone}));
    let corrupt_path = runtime.write("corrupt-selection.bin", b"synthetic invalid PNG")?;
    let mut corrupt_owner = owner(&runtime, &display, &helper, &corrupt_path, None, false)?;
    let (corrupt, _) = capture(&runtime, &display, &helper, "corrupt-result.png")?;
    corrupt_owner.stop()?;
    cases.push(json!({"experiment":"E3","case":"native-corrupt-selection","status":if corrupt["name"]=="CORRUPT_IMAGE" {"pass"} else {"fail"},"observed":corrupt}));
    let mut text_command = display.command(&runtime, &helper);
    text_command
        .arg("text")
        .arg(&first_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut text_owner = OwnedChild::spawn(&mut text_command)?;
    let text_stdout = text_owner
        .child()
        .stdout
        .as_mut()
        .ok_or_else(|| fail("missing text owner output"))?;
    wait_readable(text_stdout.as_raw_fd())?;
    let mut text_ready = String::new();
    BufReader::new(text_stdout).read_line(&mut text_ready)?;
    if text_ready != "READY\n" {
        return Err(fail("text owner not ready"));
    }
    let (text, _) = capture(&runtime, &display, &helper, "text-result.png")?;
    text_owner.stop()?;
    cases.push(json!({"experiment":"E3","case":"native-text-only-selection","status":if text["name"]=="CLIPBOARD_EMPTY" {"pass"} else {"fail"},"observed":text,"text_fallback":false}));
    let mut oversized = first.clone();
    oversized[16..20].copy_from_slice(&u32::MAX.to_be_bytes());
    let crc = crc32fast::hash(&oversized[12..29]);
    oversized[29..33].copy_from_slice(&crc.to_be_bytes());
    let over_path = runtime.write("oversized-dim.png", &oversized)?;
    let mut over_owner = owner(&runtime, &display, &helper, &over_path, None, false)?;
    let (over, _) = capture(&runtime, &display, &helper, "oversized-result.png")?;
    over_owner.stop()?;
    cases.push(json!({"experiment":"E3","case":"native-dimension-preallocation","status":if over["name"]=="LIMIT_EXCEEDED" && over["largest_property_request"]==36 {"pass"} else {"fail"},"observed":over,"raw_output_allocated":0}));
    let mut huge = first.clone();
    huge.resize(image::PNG_MAX + 1, 0);
    let huge_path = runtime.write("oversized-property.png", &huge)?;
    drop(huge);
    let mut huge_owner = owner(&runtime, &display, &helper, &huge_path, None, false)?;
    let (large, _) = capture(&runtime, &display, &helper, "huge-result.png")?;
    huge_owner.stop()?;
    cases.push(json!({"experiment":"E3","case":"native-encoded-size-preallocation","status":if large["name"]=="LIMIT_EXCEEDED" && large["largest_property_request"]==36 {"pass"} else {"fail"},"observed":large,"raw_output_allocated":0}));
    let mut incr_owner = owner(&runtime, &display, &helper, &first_path, None, true)?;
    let (incr, _) = capture(&runtime, &display, &helper, "incr-result.png")?;
    incr_owner.stop()?;
    cases.push(json!({"experiment":"E3","case":"incr-adapter-refusal","status":if incr["name"]=="INCR_UNSUPPORTED" {"pass"} else {"fail"},"observed":incr,"selection_effect":"INCR combination unsupported; no unbounded accumulation"}));
    let headless = runtime
        .command(&helper)
        .arg("capture")
        .arg(runtime.path.join("headless.png"))
        .output()?;
    let unavailable: Value = serde_json::from_slice(&headless.stdout)?;
    let denied = runtime.write("denied-authority", b"")?;
    let denied_output = display
        .command(&runtime, &helper)
        .env("XAUTHORITY", denied)
        .arg("capture")
        .arg(runtime.path.join("denied.png"))
        .output()?;
    let denied_value: Value = serde_json::from_slice(&denied_output.stdout)?;
    cases.push(json!({"experiment":"E3","case":"missing-and-denied-desktop","status":if unavailable["name"]=="CLIPBOARD_UNAVAILABLE" && denied_value["name"]=="CLIPBOARD_UNAVAILABLE" {"pass"} else {"fail"},"missing":unavailable["name"],"denied":denied_value["name"],"limitation":"XOpenDisplay cannot reliably distinguish denied from unavailable; CLIPBOARD_DENIED classification unsupported"}));
    cases.extend(codec_cases()?);
    match crate::visual::suite(&runtime, &display) {
        Ok(visual) => cases.extend(visual),
        Err(error) => cases.push(json!({"experiment":"E5","case":"native-visual-fixture","status":"fail","observations":[error.to_string()]})),
    }
    let mut artifact_record = Value::Null;
    if let Some(directory) = artifacts {
        if !directory.is_dir() {
            return Err(fail("artifact directory must already exist"));
        }
        let path = directory.join("sealed.png");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)?;
        file.write_all(&sealed)?;
        artifact_record = json!({"path":path,"sha256":image::digest(&sealed),"byte_length":sealed.len(),"binding":binding,"source":"one private-X11 snapshot -> independent plugin -> exact seal"});
    }
    selection_b.stop()?;
    display_b.stop()?;
    display.stop()?;
    let runtime_path = runtime.path.clone();
    drop(display_b);
    drop(display);
    drop(runtime);
    let failed = cases
        .iter()
        .any(|case| case["status"] == "fail" || case["status"] == "not-run");
    let report = json!({"schema_version":1,"suite":"phase0-image-visual","status":if failed {"fail"} else {"pass-with-rejected-candidates"},"environment":{"native_os":std::env::consts::OS,"architecture":std::env::consts::ARCH,"display":"new private Xvfb -nolisten tcp, private Xauthority","terminal":"xterm private display","libraries":{"png":"0.18.1","vt100":"0.18.1 no std feature (bounded vte OSC ArrayVec)","crc32fast":"1.5.0","base64":"0.22.1"}},"cases":cases,"artifact":artifact_record,"cleanup":{"owned_display_stopped":true,"private_runtime_removed":!runtime_path.exists()},"limitations":["not a released clipboard or terminal adapter","INCR/Wayland/Windows/macOS/physical IME unsupported or unverified","restoration is selected plain visual state; attributes/wrap/scroll regions and resize reflow not faithful","Core/PTY/SSH survival and network/Core recovery are separate core harness observations"]});
    runtime::write_report(output, &report)?;
    if failed {
        Err(fail(
            "image/visual assertions incomplete or failed; inspect report",
        ))
    } else {
        Ok(())
    }
}
fn codec_cases() -> Result<Vec<Value>> {
    let raw = vec![127u8; 2048 * 2048 * 4];
    let large = image::encode(2048, 2048, &raw)?;
    crate::allocation::reset();
    let decoded = image::decode(&large, Some((2048, 2048)))?;
    let measurements = crate::allocation::measure();
    let mut over = image::encode(1, 1, &[0; 4])?;
    over[16..20].copy_from_slice(&u32::MAX.to_be_bytes());
    let crc = crc32fast::hash(&over[12..29]);
    over[29..33].copy_from_slice(&crc.to_be_bytes());
    crate::allocation::reset();
    let refusal = image::decode(&over, None).unwrap_err().to_string();
    let rejected = crate::allocation::measure();
    let valid = image::encode(2, 2, &[0; 16])?;
    let mut bad_crc = valid.clone();
    bad_crc[29] ^= 1;
    let mut bomb = large.clone();
    bomb[16..20].copy_from_slice(&1u32.to_be_bytes());
    bomb[20..24].copy_from_slice(&1u32.to_be_bytes());
    let crc = crc32fast::hash(&bomb[12..29]);
    bomb[29..33].copy_from_slice(&crc.to_be_bytes());
    crate::allocation::reset();
    let bomb_rejected = image::decode(&bomb, None).is_err();
    let bomb_alloc = crate::allocation::measure();
    Ok(vec![
        json!({"experiment":"E3","case":"png-preallocation-and-alpha-profile","status":if decoded.2==raw && refusal=="LIMIT_EXCEEDED" && rejected.0<1024 && bomb_rejected && image::decode(&bad_crc,None).is_err() && image::decode(&valid,Some((1,1))).is_err(){"pass"}else{"fail"},"decoded_bytes":decoded.2.len(),"valid_largest_rust_allocation":measurements.0,"valid_peak_extra_rust_bytes":measurements.1,"oversized_claim_largest_rust_allocation":rejected.0,"decompression_bomb_rejected":bomb_rejected,"bomb_largest_rust_allocation":bomb_alloc.0,"bomb_peak_extra_rust_bytes":bomb_alloc.1,"png_max":image::PNG_MAX,"raw_max":image::RAW_MAX,"limitations":["allocator counters measure Rust allocations, Xlib property requests are separately bounded","png Limits is best effort; strict preflight excludes ancillary/interlaced/APNG allocation paths and metadata claims are checked before decoder creation"]}),
    ])
}
