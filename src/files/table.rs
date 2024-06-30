use prettytable::{format, Cell, Row, Table};

use super::{parsers, FileInfo};

pub fn create_table(files: Vec<FileInfo>) -> Table {
    let mut files_table = Table::new();
    files_table.set_format(*format::consts::FORMAT_CLEAN);

    files_table.add_row(Row::new(vec![
        Cell::new("name"),
        Cell::new("size"),
        Cell::new("created at"),
    ]));
    files_table.add_empty_row();

    for file in files {
        let formatted_size = parsers::from_bytes_to_formatted_size(file.size);
        let formatted_name = if file.extension.as_str() == "/" {
            format!("{}/", file.name)
        } else {
            format!("{}", file.name)
        };

        files_table.add_row(Row::new(vec![
            Cell::new(formatted_name.as_str()),
            Cell::new(formatted_size.as_str()),
            Cell::new(file.created_at.as_str()),
        ]));
    }

    files_table
}
