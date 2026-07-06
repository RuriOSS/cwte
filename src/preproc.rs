#[cfg(debug_assertions)]
use crate::debug;
use colored::Colorize;
use rustix::fs::{MemfdFlags, memfd_create};
use rustix::fs::{SealFlags, fcntl_add_seals};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::os::fd::AsFd;
use std::process::Command;
pub fn clang_format_prepare_layer(mut input: File) -> File {
    /*
     * clang-format the input file, and return the output file.
     * So that we will have everything in a fixed format,
     * to bypass AST parsing.
     *
     */
    // Read input to string.
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Failed to read input file");
    // Replace:
    // :<  _CE_SAD :sad path when error
    // :>  _CE_HAP :happy path when no error
    // :o  _CE_LWE :log when error
    // ::} _CE_NUS :just a todo mark
    // :D  _CE_LAF :ignore error handler forever
    // :3  _CE_DFM :do that for me, an AI-native mark
    content = content.replace(":<", "_CE_SAD");
    content = content.replace(":>", "_CE_HAP");
    content = content.replace(":o", "_CE_LWE");
    content = content.replace("::}", "_CE_NUS");
    content = content.replace(":D", "_CE_LAF");
    content = content.replace(":3", "_CE_DFM");
    // Call clang-format --assume-filename=test.c
    // Write content to clang-format's stdin,
    // and read the output from clang-format's stdout.
    let mut clang_format_child = Command::new("clang-format")
        .arg("--assume-filename=test.c")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn clang-format");
    {
        let mut stdin = clang_format_child
            .stdin
            .take()
            .expect("Failed to open stdin");
        stdin
            .write_all(content.as_bytes())
            .expect("Failed to write to stdin");
    }
    let output = clang_format_child
        .wait_with_output()
        .expect("Failed to read stdout");
    if !output.status.success() {
        eprintln!(
            "{}",
            "Error: clang-format failed. Please make sure clang-format is installed and in your PATH.".red()
        );
        std::process::exit(1);
    }
    let formatted_content = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    // Create a memfd file, and write the formatted content to it.
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    mfd_file
        .write_all(formatted_content.as_bytes())
        .expect("Failed to write to memfd");
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // For debugging, dump the memfd content to a file.
    #[cfg(debug_assertions)]
    debug::cwte_dump(
        mfd_file.try_clone().expect("Failed to clone memfd"),
        "clang_format_prepare_layer.cei",
    );
    // Return the memfd file for further processing.
    mfd_file
}
pub fn prepare_layer(mut input: File) -> File {
    /*
     * Prepare, add ce_line_xx mark to each line,
     * So we can get the line number from the mark later.
     *
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
    // For debugging, dump the memfd content to a file.
    #[cfg(debug_assertions)]
    debug::cwte_dump(
        mfd_file.try_clone().expect("Failed to clone memfd"),
        "prepare_layer.cei",
    );
    // Return the memfd file for further processing.
    mfd_file
}

pub fn clang_format_final_layer(mut input: File, show_warning: bool) -> File {
    /*
     * clang-format the input file, and return the output file.
     * So that users don't need to format again.
     */
    // Lint the input file, if has _CE_SAD, _CE_HAP, _CE_LWE, _CE_NUS, _CE_LAF, _CE_DFM, then warning.
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Failed to read input file");
    if show_warning {
        if content.contains("_CE_SAD")
            || content.contains("_CE_HAP")
            || content.contains("_CE_LWE")
            || content.contains("_CE_NUS")
            || content.contains("_CE_LAF")
        {
            eprintln!("\n{}",
            "Warning: The output file contains _CE_SAD (:<), _CE_HAP (:>), _CE_LWE (:o), _CE_NUS (::}), or _CE_LAF (:D) marks.
These marks are used for internal processing and should not appear in the final output.
Please check If cwte is working correctly, or just fire cwte.".red()
        );
        }
        if content.contains("_CE_DFM") {
            eprintln!(
            "\n{}",
            "Warning: The output file contains _CE_DFM (:3) mark. Call your LLM to do that for you."
                .red()
        );
        }
    }
    // Call clang-format --assume-filename=test.c
    // Write content to clang-format's stdin,
    // and read the output from clang-format's stdout.
    let mut clang_format_child = Command::new("clang-format")
        .arg("--assume-filename=test.c")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn clang-format");
    {
        let mut stdin = clang_format_child
            .stdin
            .take()
            .expect("Failed to open stdin");
        stdin
            .write_all(content.as_bytes())
            .expect("Failed to write to stdin");
    }
    let output = clang_format_child
        .wait_with_output()
        .expect("Failed to read stdout");
    if !output.status.success() {
        eprintln!(
            "{}",
            "Error: clang-format failed. Please make sure clang-format is installed and in your PATH.".red()
        );
        std::process::exit(1);
    }
    let mut formatted_content = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    // Replace internal marks with cwte symbols for final output.
    formatted_content = formatted_content.replace("_CE_SAD", ":<");
    formatted_content = formatted_content.replace("_CE_HAP", ":>");
    formatted_content = formatted_content.replace("_CE_LWE", ":o");
    formatted_content = formatted_content.replace("_CE_NUS", "::}");
    formatted_content = formatted_content.replace("_CE_LAF", ":D");
    formatted_content = formatted_content.replace("_CE_DFM", ":3");
    // Create a memfd file, and write the formatted content to it.
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    mfd_file
        .write_all(formatted_content.as_bytes())
        .expect("Failed to write to memfd");
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // For debugging, dump the memfd content to a file.
    #[cfg(debug_assertions)]
    debug::cwte_dump(
        mfd_file.try_clone().expect("Failed to clone memfd"),
        "clang_format_final_layer.cei",
    );
    // Return the memfd file for further processing.
    mfd_file
}

pub fn final_layer(mut input: File) -> File {
    /*
     * Finally, remove @ce_line_xx@ mark.
     * Just a simple eraser.
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
    // Now, erase the `@ce_line_xx@` in content.
    for line in content.lines() {
        writeln!(mfd_file, "{}", erase_line_no_mark(line)).expect("Failed to write to file");
    }
    // Make the memfd immutable to prevent further modification.
    mfd_file.sync_all().expect("Failed to sync memfd");
    fcntl_add_seals(mfd_file.as_fd(), SealFlags::WRITE).expect("Failed to add seals to memfd");
    // For debugging, dump the memfd content to a file.
    #[cfg(debug_assertions)]
    debug::cwte_dump(
        mfd_file.try_clone().expect("Failed to clone memfd"),
        "final_layer.cei",
    );
    // Return the memfd file for further processing.
    mfd_file
}

pub fn get_line_no(line: &str) -> Result<usize, &'static str> {
    /*
     * Get the line number from @ce_line_xx@ mark, and return it.
     * This mark is only at start of the line, @ should be the first character of the line.
     * Or if we cannot parse the line number, just return an error.
     */
    let Some(rest) = line.strip_prefix("@ce_line_") else {
        return Err("missing line mark");
    };

    let Some(end) = rest.find('@') else {
        return Err("invalid line mark");
    };

    rest[..end]
        .parse::<usize>()
        .map_err(|_| "invalid line number")
}
pub fn erase_line_no_mark(line: &str) -> String {
    /*
     * Erase the @ce_line_xx@ mark in the line, and return the fixed line.
     * This mark is only at start of the line, @ should be the first character of the line.
     * Or if we cannot find the mark, just return the original line.
     */
    if let Some(rest) = line.strip_prefix("@ce_line_") {
        if let Some((_, fixed)) = rest.split_once('@') {
            return fixed.to_string();
        }
    }
    line.to_string()
}
