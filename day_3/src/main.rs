//! Read input map from file and parse it in our format:
//! We have a Vec that contains each row
//! Each row is parsed as a BitVec where TRUE -> TREE;    FALSE -> EMPTY
//! Iterate through each row and sum up the values at the specified indicies

use bitvec::prelude::*;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input_file_path = "../day_3/src/input";

  
    let input_map = BufReader::new(
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines()
            .map(|line| // map each line into a bitvec
                line.expect("Couldn't read line")
                    .chars()
                    .map(|character|  // map each character into bool
                        if character == '.' { false } // empty space
                        else { true } // tree
                    )
                    .collect::<BitVec>()
            )
            .collect::<Vec<BitVec>>();

    let nr_trees_encountered = input_map.iter()
                                              .enumerate()
                                              .fold(0,|accumulator, (index, value)|
                                                accumulator + 
                                                value[(index*3)%value.len()] as usize
                                              );

    println!("{:?}", nr_trees_encountered);
}
