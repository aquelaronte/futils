use super::FileInfo;

mod add;
mod file_type;
mod filt;
mod search;

pub fn main(files: &mut Vec<FileInfo>) {
    add::main(files);
    filt::main(files);
    search::main(files);
    file_type::main(files);
}
