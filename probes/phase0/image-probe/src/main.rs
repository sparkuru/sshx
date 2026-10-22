mod image;
mod plugin;
mod runtime;
mod suite;
mod visual;

mod allocation {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};
    pub struct Counting;
    static LIVE: AtomicUsize = AtomicUsize::new(0);
    static BASE: AtomicUsize = AtomicUsize::new(0);
    static PEAK: AtomicUsize = AtomicUsize::new(0);
    static LARGEST: AtomicUsize = AtomicUsize::new(0);
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let pointer = System.alloc(layout);
            if !pointer.is_null() {
                let live = LIVE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
                PEAK.fetch_max(live, Ordering::Relaxed);
                LARGEST.fetch_max(layout.size(), Ordering::Relaxed);
            }
            pointer
        }
        unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
            LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
            System.dealloc(pointer, layout);
        }
        unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
            let next = System.realloc(pointer, layout, size);
            if !next.is_null() {
                let live = LIVE.fetch_add(size, Ordering::Relaxed) + size;
                LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
                PEAK.fetch_max(live.saturating_sub(layout.size()), Ordering::Relaxed);
                LARGEST.fetch_max(size, Ordering::Relaxed);
            }
            next
        }
    }
    pub fn reset() {
        let base = LIVE.load(Ordering::Relaxed);
        BASE.store(base, Ordering::Relaxed);
        PEAK.store(base, Ordering::Relaxed);
        LARGEST.store(0, Ordering::Relaxed);
    }
    pub fn measure() -> (usize, usize) {
        (
            LARGEST.load(Ordering::Relaxed),
            PEAK.load(Ordering::Relaxed)
                .saturating_sub(BASE.load(Ordering::Relaxed)),
        )
    }
}
#[global_allocator]
static ALLOCATOR: allocation::Counting = allocation::Counting;

fn run() -> image::Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.first().and_then(|arg| arg.to_str()) {
        None | Some("--help") | Some("-h") => {
            println!("phase0-image: independent public-RPC PNG encoder and native private-display probes\n\nUsage:\n  phase0-image suite --output NEW_JSON [--artifact-dir EXISTING_PRIVATE_DIR] [--binding-file CORE_BINDING_JSON]\n  phase0-image plugin\n\nThe suite creates a fresh Xvfb; it never uses the existing display.\nPlugin stdin/stdout are Content-Length JSON-RPC only.\nNative requirements: cc + Xlib headers/library, Xvfb, xterm, Vim.");
            Ok(())
        }
        Some("plugin") if args.len() == 1 => plugin::run(),
        Some("oracle-child") if args.len() == 3 => {
            let result = visual::oracle_child(
                std::path::Path::new(&args[1]),
                std::path::Path::new(&args[2]),
            );
            if let Err(error) = &result {
                let _ = runtime::write_report(
                    std::path::Path::new(&args[2]),
                    &serde_json::json!({"error":error.to_string()}),
                );
            }
            result
        }
        Some("suite") => {
            let mut output = None;
            let mut artifacts = None;
            let mut binding = None;
            let mut index = 1;
            while index < args.len() {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| image::fail("option value required"))?;
                match args[index].to_str() {
                    Some("--output") => output = Some(std::path::PathBuf::from(value)),
                    Some("--artifact-dir") => artifacts = Some(std::path::PathBuf::from(value)),
                    Some("--binding-file") => binding = Some(std::path::PathBuf::from(value)),
                    _ => return Err(image::fail("unknown option")),
                }
                index += 2;
            }
            suite::run(
                &output.ok_or_else(|| image::fail("--output required"))?,
                artifacts.as_deref(),
                binding.as_deref(),
            )
        }
        _ => Err(image::fail("unknown command; use --help")),
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("phase0-image: {error}");
        std::process::exit(1);
    }
}
