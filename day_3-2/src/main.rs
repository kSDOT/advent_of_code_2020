//! Like previous solution, but we solve the map multiple times for different pairs
//! And multiply the result of each solution
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

    let pairs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    
    let nr_trees_encountered: usize = 
        pairs.into_iter()
             .map(|index_pair|
                input_map
                    .iter()
                    .enumerate()
                    .filter(|(index, _)|
                        index % index_pair.1 == 0
                    )
                    .fold(0,|accumulator, (index, value)|
                        // ((index/index_pair.1) * index_pair.0)  is a generic solution to find out how many times we have iterated
                        // this is different from the previous solution which was hardcoded
                         accumulator + value[((index/index_pair.1) * index_pair.0) % value.len()] as usize
                    )
             )
             .product();

    println!("{:?}", nr_trees_encountered);
}