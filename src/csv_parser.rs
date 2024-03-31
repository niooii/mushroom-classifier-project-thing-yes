use std::fs;

use crate::traits::CsvParsable;

fn get_num_columns(data: &str) -> usize {
    data.chars()
        .take_while(|&ch| ch != '\n')
        .filter(|&ch| ch == ',')
        .count() + 1
}

pub struct CsvFile {
    data: String,
    num_columns: usize
}

impl CsvFile {
    pub fn new(path: &str) -> Self {
        let data = fs::read_to_string(path).unwrap();
        let num_columns = get_num_columns(&data);
        Self {
            data,
            num_columns
        }
    }
}

pub struct Column<T> {
    pub name: String,
    pub data: Vec<T>
}

impl CsvFile {
    pub fn read_column_by_idx<T>(&self, column_idx: usize) -> Option<Column<T>> 
    where T: CsvParsable
    {
        if column_idx <= self.num_columns {
            return Some(todo!())
        }
        None
    }

    pub fn read_column_by_name<T>(&self, name: &str) -> Option<Column<T>> 
    where T: CsvParsable
    {
        None
    }
}