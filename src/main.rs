/*
 * Now I scream. WTH is this QwQ?
 * Don't blame me QwQ, all rust code is written by LLMs,
 * and I have never learned rust in fact.
 */
mod debug;
mod linter;
mod nautilus;
mod preproc;
mod scmp;
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Parser)]
#[command(name = "cwte")]
#[command(version = "0.1.0")]
#[command(about = "cwte")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gen { input: String, output: String },
    Scmp { input: String, output: String },
    Fmt { input: String },
    Version {},
}

fn cwte_generator(input: &str, output: &str) {
    let input_file = fs::File::open(input).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let mut mfd_file = preproc::clang_format_prepare_layer(input_file);
    mfd_file = preproc::prepare_layer(mfd_file);
    // Process the input file with nautilus layer, and get the memfd file.
    mfd_file = nautilus::nautilus_layer(mfd_file, input);
    // Process the memfd file with linter layer, and get the new memfd file.
    mfd_file = linter::linter_layer(mfd_file, input);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = preproc::final_layer(mfd_file);
    // Format the output with clang_format_final_layer.
    mfd_file = preproc::clang_format_final_layer(mfd_file, true);
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
        "{}{}",
        "\nCwte processing completed, output written to ".green(),
        output.blue()
    );
    println!(
        "{}{}",
        "I hope I'm just a helpful tail ".green(),
        "::::<".yellow()
    );
}
fn scmp_generator(input: &str, output: &str) {
    let input_file = fs::File::open(input).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let mut mfd_file = preproc::clang_format_prepare_layer(input_file);
    mfd_file = preproc::prepare_layer(mfd_file);
    // Process the input file with scmp layer, and get the memfd file.
    mfd_file = scmp::scmp_layer(mfd_file, input);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = preproc::final_layer(mfd_file);
    // Format the output with clang_format_final_layer.
    mfd_file = preproc::clang_format_final_layer(mfd_file, true);
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
        "{}{}",
        "\nCwte processing completed, output written to ".green(),
        output.blue()
    );
    println!(
        "{}{}",
        "I hope I'm just a helpful tail ".green(),
        "::::<".yellow()
    );
}
fn cwte_fmt(input: &str) {
    let input_file = fs::File::open(input).expect("Failed to open input file");
    // Process the input file with clang_format_prepare_layer, and get the memfd file.
    let mut mfd_file = preproc::clang_format_prepare_layer(input_file);
    // Process the memfd file with clang_format_final_layer, and get the new memfd file.
    mfd_file = preproc::clang_format_final_layer(mfd_file, false);
    // Write the content of memfd to the output file.
    let mut output_file = fs::File::create(&input).expect("Failed to create output file");
    // Set output_file size to 0, so that we can overwrite the content of the file.
    output_file
        .set_len(0)
        .expect("Failed to set output file size to 0");
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
        "{}{}",
        "Cwte formatting completed, output written to ".green(),
        input.blue()
    );
    println!(
        "{}{}",
        "I hope I'm just a helpful tail ".green(),
        "::::<".yellow()
    );
}
fn main() {
    /*
     * We will never release any memfd file, kernel will help us do that.
     * Say thanks to the kernel, say thanks to memfd,
     * and have an ice cream.
     */
    /*
     * cwte has two modes, gen and scmp.
     * gen mode will generate by .hce rules,
     * and scmp mode will generate by json rules.
     */
    #[cfg(debug_assertions)]
    debug::setup_panic_hook();
    let cli = Cli::parse();
    match cli.command {
        Commands::Gen { input, output } => {
            cwte_generator(&input, &output);
        }
        Commands::Scmp { input, output } => {
            scmp_generator(&input, &output);
        }
        Commands::Fmt { input } => {
            cwte_fmt(&input);
        }
        Commands::Version {} => {
            println!("{}", "\nCwte version 0.1.0\n".green());
            println!(
                "{}",
                "C With Tailed Error handler/Cute Way To handle Error".green()
            );
            println!("{}", "But not `C Way To Evolve` :<\n".green());
            println!("{}", "         _-''''-._".blue());
            println!("{}", "       /`         `.".blue());
            println!("{}", "      /   .,~~~,.   \\".blue());
            println!("{}", "     |   /       \\   |".blue());
            println!("{}", "     |  :    :>.,/   |".blue());
            println!("{}", "     \\   \\       ,___/:<".blue());
            println!("{}", "      \".  \"-----\"   /::::<".blue());
            println!("{}", "       `.          /::::::<".blue());
            println!("{}", "         '-.____../:::::::::<\n".blue());
            println!(
                "{}",
                "\"Abstraction turns reality into a black box.\"".blue()
            );
            println!(
                "{}",
                "\"When the black box springs a leak, out comes Cthulhu.\"\n".blue()
            );
            println!(
                "{}",
                "We trust you have received the usual lecture from cwte project.".green()
            );
            println!(
                "{}",
                "It usually boils down to these three things:\n".green()
            );
            println!("{}", " #1) The tail should never wag the cat.".yellow());
            println!(
                "{}",
                " #2) Your cat's tail can also make you copy-fail.".yellow()
            );
            println!(
                "{}",
                " #3) Everything will become a fossil, nothing's absolutely evolved.\n".yellow()
            );
        }
    }
}
