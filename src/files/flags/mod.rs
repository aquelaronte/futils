use super::FileInfo;

mod file_type;
mod filt;
mod search;

pub fn main(files: &mut Vec<FileInfo>) {
    filt::main(files);
    search::main(files);
    file_type::main(files);
}
