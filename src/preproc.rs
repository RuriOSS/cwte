#[cfg(debug_assertions)]
use crate::debug;
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
     * _CE_PAN :panic when error
     * _CE_HAP :happy path when no error
     * _CE_LWE :log when error
     * _CE_NUS :just a todo mark
     * _CE_LAF :ignore error handler forever
     * _CE_DFM :do that for me, an AI-native mark
     *
     */
    // Dump input to a temporary file clang_format_prepare_layer_PID.cei
    let temp_file_path = format!(
        "cwtmp_clang_format_prepare_layer_{}.cei",
        std::process::id()
    );
    let mut temp_file = fs::File::create(&temp_file_path).expect("Failed to create temporary file");
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    let mut content = Vec::new();
    input
        .read_to_end(&mut content)
        .expect("Failed to read input file");
    temp_file
        .write_all(&content)
        .expect("Failed to write to temporary file");
    // Run the following command:
    // sed -i "s/:</_CE_PAN/g" clang_format_prepare_layer.cei
    // sed -i "s/:>/_CE_HAP/g" clang_format_prepare_layer.cei
    // sed -i "s/:o/_CE_LWE/g" clang_format_prepare_layer.cei
    // sed -i "s/::}/_CE_NUS/g" clang_format_prepare_layer.cei
    // sed -i "s/:D/_CE_LAF/g" clang_format_prepare_layer.cei
    // sed -i "s/:3/_CE_DFM/g" clang_format_prepare_layer.cei
    // clang-format -i --assume-filename=test.c clang_format_prepare_layer.cei
    Command::new("sed")
        .arg("-i")
        .arg("s/:</_CE_PAN/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/:>/_CE_HAP/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/:o/_CE_LWE/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/::}/_CE_NUS/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/:D/_CE_LAF/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/:3/_CE_DFM/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("clang-format")
        .arg("-i")
        .arg("--assume-filename=test.c")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run clang-format command");
    // Then sed back the _CE_PAN, _CE_NUS, _CE_LAF to :<, ::}, :D
    Command::new("sed")
        .arg("-i")
        .arg("s/_CE_PAN/:</g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/_CE_HAP/:>/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/_CE_LWE/:o/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/_CE_NUS/::}/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/_CE_LAF/:D/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    Command::new("sed")
        .arg("-i")
        .arg("s/_CE_DFM/:3/g")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run sed command");
    // Seek to the beginning of the temporary file.
    temp_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek temporary file");
    // Read the formatted content from the temporary file to a memfd file.
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    let mut formatted_content = Vec::new();
    let mut temp_file = fs::File::open(&temp_file_path).expect("Failed to open temporary file");
    temp_file
        .read_to_end(&mut formatted_content)
        .expect("Failed to read temporary file");
    mfd_file
        .write_all(&formatted_content)
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
    // For release, remove the temporary file.
    #[cfg(not(debug_assertions))]
    fs::remove_file(&temp_file_path).expect("Failed to remove temporary file");
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

pub fn clang_format_final_layer(mut input: File) -> File {
    /*
     * clang-format the input file, and return the output file.
     * So that users don't need to format again.
     */
    // Dump input to a temporary file clang_format_final_layer_PID.cei
    let temp_file_path = format!("cwtmp_clang_format_final_layer_{}.cei", std::process::id());
    let mut temp_file = fs::File::create(&temp_file_path).expect("Failed to create temporary file");
    input
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek input file");
    let mut content = Vec::new();
    input
        .read_to_end(&mut content)
        .expect("Failed to read input file");
    temp_file
        .write_all(&content)
        .expect("Failed to write to temporary file");
    // Run the following command:
    // clang-format -i --assume-filename=test.c clang_format_final_layer.cei
    Command::new("clang-format")
        .arg("-i")
        .arg("--assume-filename=test.c")
        .arg(&temp_file_path)
        .status()
        .expect("Failed to run clang-format command");
    // Seek to the beginning of the temporary file.
    temp_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek temporary file");
    // Read the formatted content from the temporary file to a memfd file.
    let fd = memfd_create(
        "cwte_output",
        MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING,
    )
    .expect("Failed to create memfd");
    let mut mfd_file = fs::File::from(fd);
    let mut formatted_content = Vec::new();
    let mut temp_file = fs::File::open(&temp_file_path).expect("Failed to open temporary file");
    temp_file
        .read_to_end(&mut formatted_content)
        .expect("Failed to read temporary file");
    mfd_file
        .write_all(&formatted_content)
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
    // For release, remove the temporary file.
    #[cfg(not(debug_assertions))]
    fs::remove_file(&temp_file_path).expect("Failed to remove temporary file");
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
