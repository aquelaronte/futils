use std::{
    fs::{self, OpenOptions},
    path::Path,
};

use clap::Parser;

use crate::files::{
    cli::Cli,
    parsers::{from_file_to_fileinfo, from_systemtime_to_formatted_date},
    FileInfo,
};

pub fn main(files: &mut Vec<FileInfo>) {
    let add = Cli::parse().add;

    if add.as_str().trim() == "" {
        return;
    }

    let path = Path::new(add.as_str()).to_path_buf();
    if add.ends_with('/') {
        fs::create_dir(&add).expect("Error when creating directory");
        let metadata = path.metadata().expect("Failed getting metadata");

        let file_info = FileInfo {
            name: add,
            size: metadata.len(),
            created_at: from_systemtime_to_formatted_date(
                metadata.created().expect("Error getting created at"),
            ),
            extension: String::from(""),
        };
        files.push(file_info);
    } else {
        let data = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&add)
            .expect("Error when creating file");

        let file_info = from_file_to_fileinfo(data, path).expect("Error when parsing to file info");
        files.push(file_info);
    }
}
