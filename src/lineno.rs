//use colored::*;
use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;

pub fn prepare_layer(mut input: File) -> File {
    /*
     * prepare, add ce_line_xx mark to each line.
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
    for (i, line) in content.lines().enumerate() {
        // Or, write the line to the output file.
        writeln!(mfd_file, "@ce_line_{}@{}", i + 1, line).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // Return the memfd file for further processing.
    mfd_file
}

pub fn final_layer(mut input: File) -> File {
    /*
     * Finally, remove @ce_line_xx@ mark.
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
        // The line_no is now untrustable.
        // So we just match first @ and second @, and erase it.
        if let Some(start) = line.find('@') {
            // Check if the first character is @, if not, this line is unmarked, just write it to the output file.
            if line[start..].starts_with('@') {
                if let Some(end) = line[start + 1..].find('@') {
                    let fixed = format!("{}{}", &line[..start], &line[start + end + 2..]);
                    writeln!(mfd_file, "{}", fixed).expect("Failed to write to file");
                    continue;
                }
            }
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
pub fn get_line_no(line: &str) -> Result<usize, &'static str> {
    /*
     * Get the line number from @ce_line_xx@ mark, and return it.
     */
    // Parse @ce_line_xx@ mark, and return the line number.
    if let Some(start) = line.find('@') {
        if line[start..].starts_with('@') {
            if let Some(end) = line[start + 1..].find('@') {
                let line_no_str = &line[start + 1..start + end + 1];
                if line_no_str.starts_with("ce_line_") {
                    let line_no = line_no_str[8..].parse::<usize>();
                    if let Ok(no) = line_no {
                        return Ok(no);
                    }
                }
            }
        }
    }
    Err("Invalid line number mark")
}
pub fn erase_line_no_mark(line: &str) -> String {
    /*
     * Erase the @ce_line_xx@ mark in the line, and return the fixed line.
     */
    if let Some(start) = line.find('@') {
        if line[start..].starts_with('@') {
            if let Some(end) = line[start + 1..].find('@') {
                let fixed = format!("{}{}", &line[..start], &line[start + end + 2..]);
                return fixed;
            }
        }
    }
    line.to_string()
}
