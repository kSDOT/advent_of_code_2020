//! Parse input from file and split it into separate lines
//! Each line contains one number
//! We find the first number that breaks the rule
#![feature(map_first_last)]
use std::fs;
use std::collections::{BTreeSet};

fn main() {
    let input_file_path = "../day_10/src/input";

    let jolts: BTreeSet<i64> = 
    fs::read_to_string(input_file_path)
        .expect("Couldn't open file.")
        .lines() 
        .map(|line| // for each line we extract the command
            line.parse::<i64>().unwrap()
        )
        .collect();


   let mut last = 0;
   let mut differences = (0, 1);
   for current in jolts.iter(){
        match current-last {
            1 => differences.0 +=1,
            3 => differences.1 +=1,
            _ => (),
        }
        last = *current;
   } 

   println!("{:?}\n{}", jolts,differences.0 * differences.1); 
}

