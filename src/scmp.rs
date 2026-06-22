use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;
use colored::Colorize;

pub fn scmp_layer(mut input: File, file: &str) -> File {
    /*
     * :< mark for seccomp.c in ruri.
     */
    println!(
        "{}{}{}",
        "\nProcessing ".green(),
        file.blue(),
        " with scmp layer... >w<".yellow()
    );
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
        if line.contains(":<") {
            // Replace ::} with empty string, and write the line to the output file.
            let fixed = line.replace(":<", "");
            writeln!(mfd_file, "res={}", fixed).expect("Failed to write to file");
            writeln!(mfd_file, "ruri_check_seccomp_ret(res, container->no_warnings);").expect("Failed to write to file");
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
