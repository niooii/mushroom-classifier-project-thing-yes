use crate::csv_parser::CsvFile;

mod csv_parser;
mod traits;
mod tree;

fn main() {
    println!("Hello, world!");

    let mushrooms_csv = CsvFile::new("mushrooms.csv");
    let col = mushrooms_csv.read_column_by_name::<char>("class");

    println!("{:?}", col.unwrap());
}
