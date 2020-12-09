//! We collect all the entries from the input file into a sorted vector
//! We start from the first item in the vector (which will be the smallest number) and try to sum it with every other number, beginning from the largest
//! We go from both directions and meet in the middle until we find the solution or have iterated the entire vector

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;
fn main() {
    let input_file_path = "src/input";
    let required_sum = 2020;
    let half_sum = required_sum/2; // Find the midpoint
    let mut answer = 0;

    // Read the entire file into vector
    let mut entries = BufReader::new(
                                    File::open(input_file_path).expect("Couldn't open file.")
                                        )
                                        .lines()
                                        .map(|input| input.unwrap().parse::<i32>().expect("Input from file not a vaild number"))
                                        .collect::<Vec<i32>>();
    
    entries.sort();
    // Try to shorten the range from the right: numbers which add up to more than the required_sum will be excluded from iteration
    let mut max_index = entries.len();
    
    'outer: for i in 0..max_index {
        // Start from the left
        let left_number = entries[i];

        // Until the midpoint
        if left_number <= half_sum {
            let mut j = max_index-1;
            loop {
                // Now go from the right
                let right_number = entries[j];
                // Until we reach midpoint from the right as well
                if right_number < half_sum {
                    break;
                }
                let sum = left_number + right_number;
                // Or until going further left will not produce the answer
                if sum < required_sum {
                    break;
                }
                // Exclude numbers that are too big
                else if sum > required_sum {
                    max_index -= 1;
                }
                // Answer found, assign it and break out of all loops
                else {
                    answer = left_number * right_number;
                    break 'outer;
                }
                
                // Going from the right towards the midpoint
                j-=1;
            }
        }
    }
    
    if answer==0 {println!("No answer found");} 
    else {println!("{}",answer)};
}
