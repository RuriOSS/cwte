//use colored::*;
use colored::Colorize;
use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;
// Print a nautilus with file name, line number and content.
// The `::}` is a cwte TODO note.
fn print_nautilus(file: &str, content: &str, enforce: bool) {
    println!(
        "\n{}{}{}{}:",
        "Cwte tail at ".yellow(),
        file.to_string().blue(),
        " line ".yellow(),
        crate::lineno::get_line_no(content)
            .unwrap_or(0)
            .to_string()
            .blue()
    );
    println!("{}", ">>".yellow());
    println!(
        "{}{}",
        ">>  ".yellow(),
        crate::lineno::erase_line_no_mark(content).blue()
    );
    println!("{}", ">>".yellow());
    println!(
        "{}",
        "::} Here's a nautilus, have an ice cream and write a fix,".yellow()
    );
    println!("{}", "    and don't left it to be a fossil QwQ\n".yellow());
    if enforce {
        // If enforce is true, panic to prevent compiling.
        // Cooked by rust at the beginning, now I cry.
        // `}` should be `}}` in rust fmt.
        // I miss my cprintf now.
        panic!("Cwte ::}} tail is enforced, you must fix this before compiling.");
    }
}

pub fn nautilus_layer(mut input: File, file: &str) -> File {
    /*
     * Nautilus mark ::} is cwte TODO mark.
     */
    // Seek to the beginning of the file.
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    // Read input to string.
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Failed to read input file");
    // memfd magic!
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    // Now, erase the `::}` in content, and print the nautilus for it.
    for line in content.lines() {
        // If the line contains `::}`, print the nautilus and skip this line.
        if line.contains("::}") {
            print_nautilus(file, line, false);
            // Replace ::} with empty string, and write the line to the output file.
            let fixed = line.replace("::}", "");
            writeln!(mfd_file, "{}", fixed).expect("Failed to write to file");
            continue;
        }
        // Or, write the line to the output file.
        writeln!(mfd_file, "{}", line).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // Return the memfd file for further processing.
    mfd_file
}
