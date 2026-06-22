//use colored::*;
use crate::lineno;
use colored::Colorize;
use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;
pub fn linter_layer(mut input: File, file: &str) -> File {
    /*
     * :D is cwte ignore forever mark.
     * This will bypass #[[ce_enforce(func)]] in the future.
     *
     */
    // Seek to the beginning of the file before reading.
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
    // Now, erase the `:D` in content, and print the nautilus for it.
    for line in content.lines() {
        // If the line contains `:D`, print the nautilus and skip this line.
        if line.contains(":D") {
            // !FIXME
            // print we got :D at line i, and the content of this line.
            println!(
                "\n{}{}{}{}:",
                "Cwte linter at ".yellow(),
                file.to_string().blue(),
                " line ".yellow(),
                lineno::get_line_no(line).unwrap_or(0).to_string().blue()
            );
            println!("{}", ">>".yellow());
            println!(
                "{}{}",
                ">>  ".yellow(),
                lineno::erase_line_no_mark(line).blue()
            );
            println!("{}", ">>".yellow());
            println!("{}", ":D you choose to ignore this.".yellow());
            //
            //
            // Replace :D with empty string, and write the line to the output file.
            let fixed = line.replace(":D", "");
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
