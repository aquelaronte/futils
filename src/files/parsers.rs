use std::{
    fs::{File, ReadDir},
    io::Error,
    path::PathBuf,
    time::SystemTime,
};

use chrono::{DateTime, Local};

use super::FileInfo;

pub fn from_bytes_to_formatted_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    if bytes as f64 >= TB {
        format!("{:.2} TB", bytes as f64 / TB)
    } else if bytes as f64 >= GB {
        format!("{:.2} GB", bytes as f64 / GB)
    } else if bytes as f64 >= MB {
        format!("{:.2} MB", bytes as f64 / MB)
    } else if bytes as f64 >= KB {
        format!("{:.2} KB", bytes as f64 / KB)
    } else {
        format!("{} Bytes", bytes)
    }
}

pub fn from_systemtime_to_formatted_date(time: SystemTime) -> String {
    let date_time: DateTime<Local> = time.into();
    date_time.format("%Y %B/%d %H:%M").to_string()
}

pub fn from_pathbuf_to_extension(path: &PathBuf) -> String {
    if path.is_file() {
        match path.extension() {
            Some(value) => match value.to_str() {
                None => "",
                Some(value) => value,
            },
            None => "",
        }
    } else {
        "/"
    }
    .to_string()
}

pub fn from_pathbuf_to_size(path: &PathBuf) -> Result<u64, Error> {
    let metadata = path.metadata()?;
    Ok(metadata.len())
}

pub fn from_file_to_fileinfo(file: File, path: PathBuf) -> Result<FileInfo, Error> {
    let metadata = file.metadata()?;
    let created_at = from_systemtime_to_formatted_date(metadata.created()?);

    let extension = from_pathbuf_to_extension(&path);
    let name = match path.file_name() {
        None => "",
        Some(value) => match value.to_str() {
            None => "",
            Some(value) => value,
        },
    }
    .to_string();
    let size = from_pathbuf_to_size(&path)?;

    Ok(FileInfo {
        name,
        extension,
        size,
        created_at,
    })
}

pub fn from_readdir_to_fileinfo_list(dir_files: ReadDir) -> Result<Vec<FileInfo>, Error> {
    let mut files: Vec<FileInfo> = vec![];

    for file in dir_files {
        let file = file?;
        let path = file.path();

        let created_at = from_systemtime_to_formatted_date(file.metadata()?.created()?);

        let extension = from_pathbuf_to_extension(&path);

        let name = match file.file_name().to_str() {
            None => "",
            Some(value) => value,
        }
        .to_string();

        let size = from_pathbuf_to_size(&path)?;

        files.push(FileInfo {
            name,
            extension,
            size,
            created_at,
        })
    }

    Ok(files)
}
