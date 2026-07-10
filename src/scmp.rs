#[cfg(debug_assertions)]
use crate::debug;
use crate::preproc;
//use colored::Colorize;
use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;

pub fn scmp_layer(mut input: File, _file: &str) -> File {
    /*
     * :< mark for seccomp.c in ruri.
     * It's _CE_SAD now.
     * Will be a json-driven code rewriter in the future.
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
    // Now, erase the `_CE_SAD` in content, and print the nautilus for it.
    for line in content.lines() {
        // If the line contains `_CE_SAD`, print the nautilus and skip this line.
        if line.contains("_CE_SAD") {
            let fixed = line.replace("_CE_SAD", "");
            writeln!(mfd_file, "res={}", preproc::erase_line_no_mark(&fixed))
                .expect("Failed to write to file");
            writeln!(mfd_file, "ruri_check_seccomp_ret(res);").expect("Failed to write to file");
            continue;
        }
        // Or, write the line to the output file.
        writeln!(mfd_file, "{}", line).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // For debugging, dump the memfd content to a file.
    #[cfg(debug_assertions)]
    debug::cwte_dump(
        mfd_file.try_clone().expect("Failed to clone memfd"),
        "scmp_layer.cei",
    );
    // Return the memfd file for further processing.
    mfd_file
}
