use clap::Parser;

use crate::{
    files::{Cli, FileInfo},
    utils,
};

pub fn main(files: &mut Vec<FileInfo>) {
    let regex = Cli::parse().regex;

    utils::match_file_names(files, regex);
}
