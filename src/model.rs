use std::{collections::{HashMap, HashSet}, hash::Hash};

use itertools::Itertools;

use crate::{csv_parser::CsvFile, traits::CsvParsable, tree::Tree};

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

pub struct FeatureIdxMap<T>
where T: CsvParsable + PartialEq
{
    features: Vec<T>
}

impl<T> FeatureIdxMap<T> 
where T: CsvParsable + PartialEq
{
    pub fn new() -> Self {
        Self {
            features: Vec::new()
        }
    }

    // returns the feature index
    pub fn add(&mut self, feature: T) -> usize {
        if self.features.contains(&feature) {
            panic!("feature is already in here DUMBASS");
        }

        self.features.push(feature);
        self.features.len() - 1
    }

    pub fn get_feature(&self, idx: usize) -> Option<&T> {
        self.features.get(idx)
    }

    pub fn get_idx(&self, feature: &T) -> Option<usize> {
        self.features.iter().find(|e| e == feature)
    }
}

#[derive(PartialEq)]
pub enum DesicionTreeNode {
    Condition {
        feature_idx: usize,
        threshold: f32,
        info_gain: f32,
        left_idx: usize,
        right_idx: usize
    },
    Leaf {
        class: usize
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
        // (info gain, feature_index, split_on_idx)
        let mut info_gain_vec = Vec::<(f32, usize, u8)>::new();

        for column in &all_columns {
            let entropy = calc_entropy(&column.data);
            let unique_label_counts = column.data.iter()
                .sorted()
                .dedup_with_count()
                .map(|(i, val)| (*val, i))
                // .map(|(count, _val)| count)
                .collect::<HashMap<char, usize>>();

            // what are we splitting lol 
            if unique_label_counts.len() == 1 {
                continue;
            }

            let unique_label_count_byte_vals = unique_label_counts.iter()
                .map(|(val, i)| (*val as u8, *i))
                .collect::<HashMap<_, _>>();

            let byte_iter = column.data.iter()
            .sorted()
            .map(|ch| {
                *ch as u8
            });

            println!("------ FEATURE {} ------", column.name);
            println!("entropy: {}", entropy);
            println!("number of unique labels: {}", unique_label_counts.len());
            println!("distribution of unique labels: {:?}", unique_label_count_byte_vals);

            // loop through bytes as u8 to find the highest information gain
            let info_gain_and_split_idx = unique_label_count_byte_vals.iter().clone()
                .map(|(byte, _)| *byte).sorted()
                .skip(1)
                .map(|byte| {
                    let side_1 = byte_iter.clone().filter(|e| *e < byte);
                    let side_2 = byte_iter.clone().filter(|e| *e >= byte);
                   
                    let w_1 = side_1.clone()
                    .sorted()
                    .dedup()
                    .map(|b| unique_label_count_byte_vals[&b] as f32).sum::<f32>() / column.data.len() as f32;
                    let w_entropy_1 = calc_entropy(&side_1.collect()) * w_1;

                    let w_2 = side_2.clone()
                    .sorted()
                    .dedup()
                    .map(|b| unique_label_count_byte_vals[&b] as f32).sum::<f32>() / column.data.len() as f32;
                    let w_entropy_2 = calc_entropy(&side_2.collect()) * w_2;

                    // println!("splitting by idx {}", byte);
                    // println!("weight: {}:{}", w_1, w_2);
                    // println!("weighted entropy: {}", w_entropy_1);
                    // println!();

                    (entropy - w_entropy_1 - w_entropy_2, byte)
                }
            ).max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

            let best_info_gain = info_gain_and_split_idx.0;
            let best_point_of_split = info_gain_and_split_idx.1;
            println!("best point of split: {best_point_of_split}");
            println!("max information gain: {best_info_gain}");
            println!();

            info_gain_vec.push((best_info_gain, 0/*TODO!*/, best_point_of_split));
            
        }

        info_gain_vec.sort_by(|a, b| a.0.total_cmp(&b.0));
        println!("info gain vec: {:?}", info_gain_vec);

        Self {
            tree: Tree::new(unimplemented!())
        }
    }
}