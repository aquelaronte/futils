use clap::Parser;

use crate::files::{Cli, FileInfo};

pub fn main(files: &mut Vec<FileInfo>) {
    let file_type = Cli::parse().file_type;

    if file_type == "*" {
        return;
    }

    let filtered_files = files
        .iter()
        .filter_map(|file| {
            if file.extension == file_type {
                Some(file.clone())
            } else {
                None
            }
        })
        .collect();

    *files = filtered_files
}
