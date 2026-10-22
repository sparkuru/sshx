use crate::image::{fail, Result};
use crate::runtime::{self, Display, OwnedChild, Runtime};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::num::NonZeroU16;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

pub const VT_BUDGET: usize = 16 * 1024 * 1024;

pub fn parser(rows: u16, cols: u16) -> Result<vt100::Parser> {
    let estimate = usize::from(rows)
        .checked_mul(usize::from(cols))
        .and_then(|n| n.checked_mul(2 * std::mem::size_of::<vt100::Cell>()))
        .and_then(|n| n.checked_add(128 * 1024))
        .ok_or_else(|| fail("LIMIT_EXCEEDED"))?;
    if rows > 300 || cols > 500 || estimate > VT_BUDGET {
        return Err(fail("LIMIT_EXCEEDED"));
    }
    Ok(vt100::Parser::new(
        NonZeroU16::new(rows).ok_or_else(|| fail("invalid rows"))?,
        NonZeroU16::new(cols).ok_or_else(|| fail("invalid cols"))?,
        0,
    ))
}

/// Render only selected visual state, never terminal history/OSC. Attribute and
/// wrap/scroll-region fidelity remains outside this deliberately narrow renderer.
pub fn render(screen: &vt100::Screen) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    output.extend_from_slice(if screen.alternate_screen() {
        b"\x1b[?1049h"
    } else {
        b"\x1b[?1049l"
    });
    output.extend_from_slice(b"\x1b[0m\x1b[2J\x1b[H");
    let (rows, cols) = screen.size();
    for row in 0..rows.get() {
        write!(output, "\x1b[{};1H", row + 1)?;
        for col in 0..cols.get() {
            let cell = screen.cell(row, col).ok_or_else(|| fail("missing cell"))?;
            if cell.is_wide_continuation() {
                continue;
            }
            let text = cell.contents();
            if text.chars().any(char::is_control) {
                return Err(fail("unsafe cell"));
            }
            if text.is_empty() {
                output.push(b' ');
            } else {
                output.extend_from_slice(text.as_bytes());
            }
        }
    }
    let (row, col) = screen.cursor_position();
    write!(output, "\x1b[{};{}H", row + 1, col + 1)?;
    output.extend_from_slice(if screen.hide_cursor() {
        b"\x1b[?25l"
    } else {
        b"\x1b[?25h"
    });
    output.extend_from_slice(if screen.bracketed_paste() {
        b"\x1b[?2004h"
    } else {
        b"\x1b[?2004l"
    });
    output.extend_from_slice(if screen.application_cursor() {
        b"\x1b[?1h"
    } else {
        b"\x1b[?1l"
    });
    output.extend_from_slice(if screen.application_keypad() {
        b"\x1b="
    } else {
        b"\x1b>"
    });
    if output.len() > 1024 * 1024 {
        return Err(fail("DISPLAY_INCOMPLETE"));
    }
    Ok(output)
}

fn normalize(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .replace('\r', "")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_owned()
}

fn oracle(
    runtime: &Runtime,
    display: &Display,
    name: &str,
    before: &[u8],
    resize: Option<(u16, u16)>,
    after: &[u8],
) -> Result<Value> {
    let spec = runtime.write(&format!("{name}.spec.json"), &serde_json::to_vec(&json!({"before":STANDARD.encode(before),"after":STANDARD.encode(after),"resize":resize}))?)?;
    let result = runtime.path.join(format!("{name}.oracle.json"));
    let dump = runtime.path.join(format!("{name}.screen"));
    let mut command = display.command(runtime, "/usr/bin/xterm");
    // The printer command contains only a fixed executable and our generated
    // hex-named /tmp path, never a user-controlled shell word.
    let printer = format!("XTerm*printerCommand: /usr/bin/tee {}", dump.display());
    command
        .args([
            "-geometry",
            "40x12",
            "-fa",
            "monospace",
            "-fs",
            "10",
            "-xrm",
        ])
        .arg(printer)
        .args([
            "-xrm",
            "XTerm*printerAutoClose: true",
            "-xrm",
            "XTerm*printAttributes: 0",
            "-xrm",
            "XTerm*printNewline: true",
            "-xrm",
            "XTerm*allowWindowOps: true",
            "-xrm",
            "XTerm*allowSendEvents: false",
            "-e",
        ])
        .arg(std::env::current_exe()?)
        .arg("oracle-child")
        .arg(&spec)
        .arg(&result)
        .stdin(Stdio::null())
        .stdout(File::create(&dump)?)
        .stderr(File::create(runtime.path.join(format!("{name}.stderr")))?);
    let mut child = OwnedChild::spawn(&mut command)?;
    let deadline = Instant::now() + Duration::from_secs(6);
    while Instant::now() < deadline {
        if result.is_file() {
            if let Ok(observed) = serde_json::from_slice::<Value>(&fs::read(&result)?) {
                if let Some(error) = observed.get("error") {
                    return Err(fail(&format!("oracle child failed: {error}")));
                }
            }
        }
        if result.is_file() && fs::metadata(&dump).is_ok_and(|meta| meta.len() > 0) {
            break;
        }
        if child.child().try_wait()?.is_some() {
            let diagnostic = fs::read_to_string(runtime.path.join(format!("{name}.stderr")))?;
            return Err(fail(&format!(
                "xterm oracle exited before observation (result={}, dump={}): {}",
                result.exists(),
                fs::metadata(&dump).map(|value| value.len()).unwrap_or(0),
                diagnostic.chars().take(1024).collect::<String>()
            )));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    if !result.is_file() || fs::metadata(&dump)?.len() == 0 {
        return Err(fail("xterm oracle timeout"));
    }
    let mut observed: Value = serde_json::from_slice(&fs::read(&result)?)?;
    if let Some(error) = observed.get("error") {
        return Err(fail(&format!("oracle child failed: {error}")));
    }
    observed["screen"] = json!(normalize(&fs::read(&dump)?));
    fs::write(result.with_extension("exit"), [])?;
    let _ = child.wait(Duration::from_secs(2))?;
    Ok(observed)
}

type VisualCase<'a> = (&'a str, &'a [u8], Option<(u16, u16)>, &'a [u8]);
pub fn suite(runtime: &Runtime, display: &Display) -> Result<Vec<Value>> {
    let cases: Vec<VisualCase<'_>> = vec![
        (
            "normal-cursor-modes",
            b"\x1b[2J\x1b[Halpha\x1b[3;4Hbeta\x1b[?2004h\x1b[?25l\x1b[?1h",
            None,
            b"",
        ),
        (
            "alternate-screen",
            b"normal\x1b[?1049h\x1b[2J\x1b[HALTERNATE\x1b[5;7Hcursor",
            None,
            b"",
        ),
        (
            "resize-reflow",
            b"\x1b[2J\x1b[H012345678901234567890123456789012345678901234567890123456789",
            Some((20, 12)),
            b"",
        ),
        (
            "resize-explicit-redraw",
            b"before",
            Some((30, 10)),
            b"\x1b[2J\x1b[Hredrawn\x1b[4;6Hposition",
        ),
        (
            "clipboard-side-effect-not-replayed",
            b"\x1b[2J\x1b[Hsafe\x1b]52;c;c3ludGhldGlj\x07\x1b]0;synthetic title\x07",
            None,
            b"",
        ),
    ];
    let mut results = Vec::new();
    for (name, before, resize, after) in cases {
        let independent = oracle(
            runtime,
            display,
            &format!("{name}-baseline"),
            before,
            resize,
            after,
        )?;
        let mut model = parser(12, 40)?;
        model.process(before);
        if let Some((cols, rows)) = resize {
            model.set_size(
                NonZeroU16::new(rows).unwrap(),
                NonZeroU16::new(cols).unwrap(),
            );
        }
        model.process(after);
        let (rows, cols) = model.screen().size();
        let snapshot = render(model.screen())?;
        let restored = oracle(
            runtime,
            display,
            &format!("{name}-restored"),
            b"",
            Some((cols.get(), rows.get())),
            &snapshot,
        )?;
        let model_text = normalize(model.screen().contents().as_bytes());
        let screen_equal =
            independent["screen"] == restored["screen"] && independent["screen"] == model_text;
        let cursor_equal = independent["cursor"] == restored["cursor"];
        let modes_equal = independent["modes"] == restored["modes"];
        let baseline_model_cursor = independent["cursor"]
            == json!([
                model.screen().cursor_position().0 + 1,
                model.screen().cursor_position().1 + 1
            ]);
        let faithful = screen_equal && cursor_equal && modes_equal && baseline_model_cursor;
        let expected_rejection = name == "resize-reflow";
        results.push(json!({"experiment":"E5","case":name,"status":if faithful {"pass"} else if expected_rejection {"rejected-candidate"} else {"fail"},"scope":"native private-Xvfb xterm; bounded selected visual state","screen_equal":screen_equal,"cursor_equal":cursor_equal,"modes_equal":modes_equal,"model_cursor_equal":baseline_model_cursor,"baseline_screen_sha256":crate::image::digest(independent["screen"].as_str().unwrap_or("").as_bytes()),"restored_screen_sha256":crate::image::digest(restored["screen"].as_str().unwrap_or("").as_bytes()),"baseline_cursor":independent["cursor"],"restored_cursor":restored["cursor"],"baseline_modes":independent["modes"],"restored_modes":restored["modes"],"snapshot_bytes":snapshot.len(),"contains_osc":snapshot.windows(2).any(|bytes| bytes == b"\x1b]"),"limitation":if expected_rejection {"width truncation matches this tested xterm398 case; reflow emulators remain unverified"} else {"attributes, wrap flags, hidden normal-screen state and scroll regions are not faithfully restored by this limited renderer"}}));
        if name == "alternate-screen" {
            let original_return = oracle(
                runtime,
                display,
                "alternate-normal-original",
                before,
                None,
                b"\x1b[?1049l",
            )?;
            let restored_return = oracle(
                runtime,
                display,
                "alternate-normal-restored",
                &snapshot,
                None,
                b"\x1b[?1049l",
            )?;
            let equal = original_return["screen"] == restored_return["screen"];
            results.push(json!({"experiment":"E5","case":"alternate-return-normal","status":if equal {"pass"} else {"rejected-candidate"},"screen_equal":equal,"limitation":"visible alternate snapshot does not retain the hidden normal buffer; this renderer is rejected as a full attach strategy"}));
        }
    }
    let restricted = oracle(
        runtime,
        display,
        "restricted-redraw",
        b"\x1b[2J\x1b[Hbefore-detach",
        None,
        b"\x1b[2J\x1b[Hverified-fixture-redraw\x1b[2;1Hready",
    )?;
    results.push(json!({"experiment":"E5","case":"restricted-redraw","status":if restricted["screen"] == "verified-fixture-redraw\nready" {"pass"} else {"fail"},"limitation":"requires explicit cooperating receiver redraw; arbitrary alternate-screen/editor restoration unsupported"}));
    crate::allocation::reset();
    let mut bounded = parser(300, 500)?;
    let chunk = [b'x'; 65536];
    for _ in 0..128 {
        bounded.process(&chunk);
    }
    bounded.process(b"\x1b[?1049h");
    bounded.process(&chunk);
    let measured = crate::allocation::measure();
    let refused = parser(301, 500).is_err();
    results.push(json!({"experiment":"E5","case":"detached-output-bounded","status":if measured.1<=VT_BUDGET && refused {"pass"}else{"fail"},"input_bytes":chunk.len()*129,"scrollback_capacity":0,"visible_cells":150000,"retained_render_bytes":render(bounded.screen())?.len(),"cell_bytes":std::mem::size_of::<vt100::Cell>(),"largest_rust_allocation":measured.0,"peak_extra_rust_bytes":measured.1,"vt_budget":VT_BUDGET,"dimensions_over_limit_refused":refused,"limitations":["no terminal history retained","Core/SSH process survival and network/Core failure evidence are separate core harness cases"]}));
    results.extend(editor(runtime, display)?);
    Ok(results)
}

struct TerminalMode(libc::termios);
impl Drop for TerminalMode {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(0, libc::TCSANOW, &self.0);
        }
    }
}
fn query(bytes: &[u8], end: u8) -> Result<Vec<u8>> {
    std::io::stdout().write_all(bytes)?;
    std::io::stdout().flush()?;
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut answer = Vec::new();
    while Instant::now() < deadline && answer.len() < 128 {
        let mut fd = libc::pollfd {
            fd: 0,
            events: libc::POLLIN,
            revents: 0,
        };
        if unsafe { libc::poll(&mut fd, 1, 20) } <= 0 {
            continue;
        }
        // std::io::Stdin has an internal buffer; polling its kernel FD after
        // consuming one byte can miss an already-buffered remainder.
        let mut byte = [0u8];
        let count = unsafe { libc::read(0, byte.as_mut_ptr().cast(), 1) };
        if count != 1 {
            return Err(fail("xterm query EOF"));
        }
        answer.push(byte[0]);
        if byte[0] == end {
            return Ok(answer);
        }
    }
    Err(fail("xterm query timeout"))
}
pub fn oracle_child(spec: &Path, output: &Path) -> Result<()> {
    let mut saved = unsafe { std::mem::zeroed::<libc::termios>() };
    if unsafe { libc::tcgetattr(0, &mut saved) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let _restore = TerminalMode(saved);
    let mut raw = saved;
    unsafe {
        libc::cfmakeraw(&mut raw);
    }
    if unsafe { libc::tcsetattr(0, libc::TCSANOW, &raw) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let spec: Value = serde_json::from_slice(&fs::read(spec)?)?;
    let before = STANDARD.decode(
        spec["before"]
            .as_str()
            .ok_or_else(|| fail("missing before"))?,
    )?;
    std::io::stdout().write_all(&before)?;
    if let Some(size) = spec["resize"].as_array() {
        let cols = size[0].as_u64().unwrap_or(0) as u16;
        let rows = size[1].as_u64().unwrap_or(0) as u16;
        write!(std::io::stdout(), "\x1b[8;{rows};{cols}t")?;
        std::io::stdout().flush()?;
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let mut actual = unsafe { std::mem::zeroed::<libc::winsize>() };
            if unsafe { libc::ioctl(0, libc::TIOCGWINSZ, &mut actual) } == 0
                && actual.ws_col == cols
                && actual.ws_row == rows
            {
                break;
            }
            if Instant::now() >= deadline {
                return Err(fail("xterm resize timeout"));
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    std::io::stdout().write_all(
        &STANDARD.decode(
            spec["after"]
                .as_str()
                .ok_or_else(|| fail("missing after"))?,
        )?,
    )?;
    let cursor_raw = query(b"\x1b[6n", b'R')?;
    let cursor_text = std::str::from_utf8(&cursor_raw)?;
    // A captured editor may have sent color/terminal-feature queries. Their
    // ordered replies precede our explicit CPR; use only its final CSI response.
    let cursor_payload = cursor_text
        .rsplit_once("\x1b[")
        .map(|(_, payload)| payload)
        .ok_or_else(|| fail("cursor response missing CSI"))?;
    let cursor: Vec<u16> = cursor_payload
        .trim_end_matches('R')
        .split(';')
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| fail(&format!("unexpected cursor response: {cursor_raw:02x?}")))?;
    let mut modes = serde_json::Map::new();
    for mode in [1, 25, 1049, 2004] {
        let reply = query(format!("\x1b[?{mode}$p").as_bytes(), b'y')?;
        modes.insert(mode.to_string(), json!(String::from_utf8(reply)?));
    }
    std::io::stdout().write_all(b"\x1b[0i\x1b[4i")?;
    std::io::stdout().flush()?;
    runtime::write_report(output, &json!({"cursor":cursor,"modes":modes}))?;
    let deadline = Instant::now() + Duration::from_secs(5);
    while !output.with_extension("exit").exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

fn editor(runtime: &Runtime, display: &Display) -> Result<Vec<Value>> {
    let fixture = runtime.write(
        "editor.txt",
        b"SSHX synthetic editor\nalpha beta gamma\nthird line\n",
    )?;
    let ready = runtime.path.join("vim.ready");
    let mut master_fd = -1;
    let mut slave_fd = -1;
    let size = libc::winsize {
        ws_row: 12,
        ws_col: 40,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if unsafe {
        libc::openpty(
            &mut master_fd,
            &mut slave_fd,
            std::ptr::null_mut(),
            std::ptr::null(),
            &size,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    let mut master = unsafe { File::from_raw_fd(master_fd) };
    let slave = unsafe { File::from_raw_fd(slave_fd) };
    // This child needs setsid, so construct its Command without Runtime's setpgid.
    let mut native = std::process::Command::new("/usr/bin/vim");
    native
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", &runtime.path)
        .env("LANG", "C.UTF-8")
        .env("TERM", "xterm-256color")
        .current_dir(&runtime.path)
        .args([
            "-Nu",
            "NONE",
            "-n",
            "-i",
            "NONE",
            "-N",
            "--noplugin",
            "-c",
            "set noshowmode noruler laststatus=0 t_u7= t_RV=",
            "-c",
            "redraw!",
            "-c",
        ])
        .arg(format!("call writefile(['ready'], '{}')", ready.display()))
        .arg(&fixture)
        .stdin(slave.try_clone()?)
        .stdout(slave.try_clone()?)
        .stderr(slave);
    unsafe {
        native.pre_exec(|| {
            if libc::setsid() < 0 || libc::ioctl(0, libc::TIOCSCTTY, 0) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = OwnedChild::spawn(&mut native)?;
    let baseline = drain_editor(&mut master, &ready, Duration::from_secs(4))?;
    if !baseline
        .windows(b"SSHX synthetic editor".len())
        .any(|bytes| bytes == b"SSHX synthetic editor")
    {
        return Err(fail("Vim fixture content not observed"));
    }
    let alive_before = child.child().try_wait()?.is_none();
    let resized = libc::winsize {
        ws_row: 10,
        ws_col: 30,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if unsafe { libc::ioctl(master.as_raw_fd(), libc::TIOCSWINSZ, &resized) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let redraw_ready = runtime.path.join("vim.redrawn");
    write!(
        master,
        ":redraw!|call writefile(['redrawn'], '{}')\r",
        redraw_ready.display()
    )?;
    let after = drain_editor(&mut master, &redraw_ready, Duration::from_secs(4))?;
    let alive_after = child.child().try_wait()?.is_none();
    let mut model = parser(12, 40)?;
    model.process(&baseline);
    model.set_size(NonZeroU16::new(10).unwrap(), NonZeroU16::new(30).unwrap());
    model.process(&after);
    let independent = oracle(
        runtime,
        display,
        "vim-baseline",
        &baseline,
        Some((30, 10)),
        &after,
    )?;
    let safe = render(model.screen())?;
    let restored = oracle(runtime, display, "vim-restored", b"", Some((30, 10)), &safe)?;
    let redraw_barrier = runtime.path.join("vim.redraw-barrier");
    write!(
        master,
        "\u{c}:call writefile(['done'], '{}')\r",
        redraw_barrier.display()
    )?;
    let redraw = drain_editor(&mut master, &redraw_barrier, Duration::from_secs(4))?;
    let redraw_screen = oracle(
        runtime,
        display,
        "vim-redraw-only",
        b"",
        Some((30, 10)),
        &redraw,
    )?;
    master.write_all(b":qa!\r")?;
    let exited = child.wait(Duration::from_secs(2))?.success();
    let screen_equal = independent["screen"] == restored["screen"];
    let cursor_equal = independent["cursor"] == restored["cursor"];
    let modes_equal = independent["modes"] == restored["modes"];
    Ok(vec![
        json!({"experiment":"E5","case":"controlled-vim-native-pty","status":if alive_before && alive_after && exited && screen_equal && cursor_equal && modes_equal {"pass"} else {"rejected-candidate"},"child_alive_before_resize":alive_before,"child_alive_after_resize":alive_after,"normal_exit":exited,"screen_equal":screen_equal,"cursor_equal":cursor_equal,"modes_equal":modes_equal,"baseline_screen_sha256":crate::image::digest(independent["screen"].as_str().unwrap_or("").as_bytes()),"restored_screen_sha256":crate::image::digest(restored["screen"].as_str().unwrap_or("").as_bytes()),"restricted_redraw_screen_equal":redraw_screen["screen"]==independent["screen"],"baseline_bytes":baseline.len(),"after_resize_bytes":after.len(),"redraw_bytes":redraw.len(),"limitation":"Vim on native task-owned PTY, independent private xterm oracle; no attributes/wrap fidelity or generic arbitrary-editor redraw promise"}),
    ])
}

fn drain_editor(master: &mut File, barrier: &Path, timeout: Duration) -> Result<Vec<u8>> {
    let deadline = Instant::now() + timeout;
    let mut bytes = Vec::new();
    while Instant::now() < deadline {
        let mut fd = libc::pollfd {
            fd: master.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        let ready = unsafe { libc::poll(&mut fd, 1, 100) };
        if ready > 0 {
            let mut chunk = [0u8; 8192];
            let size = master.read(&mut chunk)?;
            if size == 0 {
                break;
            }
            if bytes.len() + size > 1024 * 1024 {
                return Err(fail("editor output budget exceeded"));
            }
            bytes.extend_from_slice(&chunk[..size]);
        } else if barrier.exists() {
            return Ok(bytes);
        }
    }
    Err(fail("editor barrier timeout"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe_visual_renderer_never_replays_osc() {
        let mut model = parser(12, 40).unwrap();
        model.process(b"visible\x1b]52;c;payload\x07\x1b]0;title\x07\x1b[?2004h");
        let rendered = render(model.screen()).unwrap();
        assert!(!rendered.windows(2).any(|part| part == b"\x1b]"));
        let mut fresh = parser(12, 40).unwrap();
        fresh.process(&rendered);
        assert_eq!(
            normalize(fresh.screen().contents().as_bytes()),
            normalize(model.screen().contents().as_bytes())
        );
        assert_eq!(
            fresh.screen().cursor_position(),
            model.screen().cursor_position()
        );
        assert!(fresh.screen().bracketed_paste());
        assert!(parser(301, 500).is_err());
    }
}
