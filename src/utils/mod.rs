use regex::Regex;

use crate::files::FileInfo;

pub fn compile_string_to_regex(value: &String) -> Regex {
    match Regex::new(value) {
        Ok(filter) => filter,
        Err(_) => {
            panic!("Error when trying to compile regex");
        }
    }
}

pub fn match_file_names(files: &mut Vec<FileInfo>, filter: String) {
    let regex = compile_string_to_regex(&filter);

    let matched_files = files
        .iter()
        .filter_map(|file| {
            if regex.is_match(&file.name) {
                Some(file.clone())
            } else {
                None
            }
        })
        .collect();

    *files = matched_files
}
