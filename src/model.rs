use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

use crate::{csv_parser::CsvFile, tree::Tree};

pub fn calc_entropy<T>(labels: &Vec<T>) -> f32
where T: PartialEq + Eq + Hash + Clone + Ord
{
    let label_counts = labels.iter()
        .sorted()
        .dedup_with_count()
        .map(|(count, _val)| count);

    let total_labels = labels.iter().count();

    label_counts.map(|label_count| {
        label_count as f32 / total_labels as f32 * (1_f32 / (label_count as f32 / total_labels as f32)).log2()
    }).sum()
} 

#[derive(PartialEq)]
pub enum DesicionTreeNode {
    Condition {
        feature_idx: usize,
        threshold: f32,
        info_gain: f32,
        left_node_idx: usize,
        right_node_idx: usize
    },
    Leaf {
        value: usize
    }
}

pub struct DesicionTree {
    tree: Tree<DesicionTreeNode>
}

impl DesicionTree {
    // Builds a desicion tree classifier given a dataset
    // I'm too lazy to make it a generic despite everything else being so, maybe i'll do it later
    pub fn new(csv_file: &CsvFile) -> Self {

        let all_columns = csv_file.read_all_columns::<char>();

        for column in &all_columns {
            let entropy = calc_entropy(&column.data);
            let unique_label_counts = column.data.iter()
                .sorted()
                .dedup_with_count()
                // .map(|(count, _val)| count)
                .collect::<Vec<_>>();

            println!("------ COLUMN {} ------", column.name);
            println!("entropy: {}", entropy);
            println!("number of unique labels: {}", unique_label_counts.len());
            println!("distribution of unique labels: {:?}", unique_label_counts);
            println!();
        }

        Self {
            tree: Tree::new(unimplemented!())
        }
    }
}