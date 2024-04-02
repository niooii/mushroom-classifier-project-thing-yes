use crate::{csv_parser::CsvFile, model::{calc_entropy, DesicionTree}, tree::Tree};

mod csv_parser;
mod traits;
mod tree;
mod model;

fn main() {
    println!("Hello, world!");
        
    let mushrooms_csv = CsvFile::new("mushrooms.csv");
    
    let desicion_tree = DesicionTree::new(&mushrooms_csv);
}
