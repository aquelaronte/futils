use clap::Parser;

use crate::{
    files::{Cli, FileInfo},
    utils,
};

pub fn main(files: &mut Vec<FileInfo>) {
    let search = Cli::parse().search;

    let regex_str = format!(".*{}.*", &search);
    utils::match_file_names(files, regex_str);
}
