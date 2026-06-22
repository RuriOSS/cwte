/*
 * Now I scream. WTH is this QwQ?
 * Don't blame me QwQ, all rust code is written by LLMs,
 * and I have never learned rust in fact.
 */
mod lineno;
mod linter;
mod nautilus;
mod scmp;
use clap::{Parser, Subcommand};
use colored::*;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Parser)]
#[command(name = "cwte")]
#[command(version = "0.1.0")]
#[command(about = "Cwte")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gen { input: String },
    Scmp { input: String },
}

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
fn cwte_generator(input: &str, output: &str) {
    let input_file = fs::File::open(input).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let mut mfd_file = lineno::prepare_layer(input_file);
    // Process the input file with nautilus layer, and get the memfd file.
    mfd_file = nautilus::nautilus_layer(mfd_file, input);
    // Process the memfd file with linter layer, and get the new memfd file.
    mfd_file = linter::linter_layer(mfd_file, input);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = lineno::final_layer(mfd_file);
    // Write the content of memfd to the output file.
    let mut output_file = fs::File::create(&output).expect("Failed to create output file");
    let mut memfd_content = Vec::new();
    mfd_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek memfd");
    mfd_file
        .read_to_end(&mut memfd_content)
        .expect("Failed to read memfd");
    output_file
        .write_all(&memfd_content)
        .expect("Failed to write to output file");
    println!(
        "{}{}{}",
        "\nCwte processing completed. Output written to ".green(),
        output.blue(),
        " >w<!!!".yellow()
    );
    println!("{}", "I hope I'm just a cute tail...".green());
}
fn scmp_generator(input: &str, output: &str) {
    let input_file = fs::File::open(input).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let mut mfd_file = lineno::prepare_layer(input_file);
    // Process the input file with scmp layer, and get the memfd file.
    mfd_file = scmp::scmp_layer(mfd_file, input);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = lineno::final_layer(mfd_file);
    // Write the content of memfd to the output file.
    let mut output_file = fs::File::create(&output).expect("Failed to create output file");
    let mut memfd_content = Vec::new();
    mfd_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek memfd");
    mfd_file
        .read_to_end(&mut memfd_content)
        .expect("Failed to read memfd");
    output_file
        .write_all(&memfd_content)
        .expect("Failed to write to output file");
    println!(
        "{}{}{}",
        "\nCwte processing completed. Output written to ".green(),
        output.blue(),
        " >w<!!!".yellow()
    );
    println!("{}", "I hope I'm just a cute tail...".green());
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
        println!("Usage: {} [args] <file>", args[0]);
        return;
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::Gen { input } => {
            let output = format!("{}.c", input);
            cwte_generator(&input, &output);
        }
        Commands::Scmp { input } => {
            let output = format!("{}.c", input);
            scmp_generator(&input, &output);
        }
    }
}
