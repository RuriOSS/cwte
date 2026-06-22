/*
 * Now I scream. WTH is this QwQ?
 * Don't blame me QwQ, all rust code is written by LLMs,
 * and I have never learned rust in fact.
 */
mod lineno;
mod linter;
mod nautilus;
use colored::*;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

// Add a hook for testing build,
// when any panic, print /proc/pid/fd,
// and sleep to freeze forever to just wait user to kill it.
#[cfg(debug_assertions)]
fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("Panic occurred: {}", info);
        let pid = std::process::id();
        eprintln!("Listing /proc/{}/fd:", pid);
        if let Ok(entries) = fs::read_dir(format!("/proc/{}/fd", pid)) {
            for entry in entries.flatten() {
                if let Ok(target) = fs::read_link(entry.path()) {
                    eprintln!(
                        "{} -> {}",
                        entry.file_name().to_string_lossy(),
                        target.display()
                    );
                }
            }
        }
        eprintln!("Freezing forever. Waiting to be killed...");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    }));
}

fn main() {
    /*
     * We will never release any memfd file, kernel will help us do that.
     * Say thanks to the kernel, say thanks to memfd,
     * and have an ice cream.
     */
    #[cfg(debug_assertions)]
    setup_panic_hook();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }
    // Open the input file.
    let input_file = fs::File::open(&args[1]).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let mut mfd_file = lineno::prepare_layer(input_file);
    // Process the input file with nautilus layer, and get the memfd file.
    mfd_file = nautilus::nautilus_layer(mfd_file, &args[1]);
    // Process the memfd file with linter layer, and get the new memfd file.
    mfd_file = linter::linter_layer(mfd_file, &args[1]);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = lineno::final_layer(mfd_file);
    // Write the content of memfd to the output file.
    let output_file = format!("{}.c", args[1]);
    let mut output = fs::File::create(&output_file).expect("Failed to create output file");
    let mut memfd_content = Vec::new();
    mfd_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek memfd");
    mfd_file
        .read_to_end(&mut memfd_content)
        .expect("Failed to read memfd");
    output
        .write_all(&memfd_content)
        .expect("Failed to write to output file");
    println!(
        "{}{}{}",
        "\nCwte processing completed. Output written to ".green(),
        output_file.blue(),
        " >w<!!!".yellow()
    );
    println!("{}", "I hope I'm just a cute tail...".green());
}
