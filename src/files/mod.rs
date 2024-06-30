mod cli;
mod flags;
mod parsers;
mod table;

use clap::Parser;
use cli::Cli;
use std::fs::{self};

#[derive(Clone)]
pub struct FileInfo {
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub created_at: String,
}

pub fn main() {
    let path = Cli::parse().path;

    let raw_files = fs::read_dir(path).expect("Error when reading dir");

    let mut files =
        parsers::from_readdir_to_fileinfo_list(raw_files).expect("Error when parsing files");

    flags::main(&mut files);

    let table = table::create_table(files);

    table.printstd()
}
