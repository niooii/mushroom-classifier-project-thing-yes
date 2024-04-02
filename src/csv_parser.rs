use std::{collections::HashMap, fs};
use itertools::Itertools;

// TODO! reduce memory footprint later maybe?
use crate::traits::CsvParsable;  

fn get_num_columns(data: &str) -> usize {
    data.chars()
        .take_while(|&ch| ch != '\n')
        .filter(|&ch| ch == ',')
        .count() + 1
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Column<T> {
    pub name: String,
    pub data: Vec<T>
}

impl CsvFile {
    pub fn read_column_by_idx<T>(&self, column_idx: usize) -> Option<Column<T>> 
    where T: CsvParsable
    {
        if column_idx <= self.num_columns {
            let mut lines_iter = self.data.lines();
            let first_line = lines_iter.next().unwrap();
            let mut idx_iter = first_line.chars()
                .enumerate()
                .filter(|(_i, ch)| *ch == ',')
                .map(|(i, _ch)| i);

            let idx_1 = if column_idx == 0 {
                0
            } else {
                idx_iter.nth(column_idx - 1).unwrap() + 1
            };
            let idx_2 = idx_iter.next().unwrap_or(first_line.len());
            let name = &first_line[idx_1..idx_2];

            let mut data = Vec::new();
            for line in lines_iter {
                let mut idx_iter = line.chars()
                .enumerate()
                .filter(|(_i, ch)| *ch == ',')
                .map(|(i, _ch)| i);

                let idx_1 = if column_idx == 0 {
                    0
                } else {
                    idx_iter.nth(column_idx - 1).unwrap() + 1
                };
                let idx_2 = idx_iter.next().unwrap_or(line.len());

                let val = T::from_bytes(&line[idx_1..idx_2].as_bytes());

                data.push(val);
            }

            return Some(
                Column {
                    name: name.to_string(),
                    data
                }
            )
        }
        None
    }

    pub fn read_column_by_name<T>(&self, name: &str) -> Option<Column<T>> 
    where T: CsvParsable
    {
        let split_idx = self.data.find(name);
        split_idx.map(|byte_idx| {
            let col_idx = 
            self.data[0..byte_idx]
                .chars()
                .filter(|ch| *ch == ',')
                .count();
            self.read_column_by_idx(col_idx)
        })?
    }

    pub fn read_all_columns<T>(&self) -> Vec<Column<T>> 
    where T: CsvParsable
    {
        (0..self.num_columns).into_iter()
        .map(|idx| self.read_column_by_idx(idx).unwrap())
        .collect()
    }

    pub fn read_row<T>(&self, row_idx: usize) -> Vec<T> {
        todo!()
    }
}