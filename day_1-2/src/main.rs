//! We reuse the solution from day_1
//! We start from the right until we hit the midpoint and calculate what the difference between the current integer and the answer
//! We try to find if this difference can be expressed as sum of two other numbers 

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;

/// The solution from day_1 refactored as a function
fn find_sum_of_two(entries: &[i32], required_sum: i32) -> Option<(i32, i32)> {
    let half_sum = required_sum/2;
    
    // Try to shorten the range from the right: numbers which add up to more than the required_sum will be excluded from iteration
    let mut max_index = entries.len();
    
    for i in 0..max_index {
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
                    return Some((left_number, right_number));
                }
                
                // Going from the right towards the midpoint
                j-=1;
            }
        }
    }

    return None;
}


fn main() {
    let input_file_path = "../day_1/src/input";
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
    
    let mut maximum_index = entries.len()-1;
    
    // Trim all the large values that can't possibly be answers
    {
        let sum_of_minimum: i32 = entries.get(0).unwrap_or(&0) + entries.get(1).unwrap_or(&0);

        loop {
            if entries[maximum_index] + sum_of_minimum <= required_sum {
                break;
            }

            maximum_index-=1;
        }
    
    }
    maximum_index+=1;

    // Iterate through each integer from the biggest to the smallest and try to find if it can be summed with 
    // 2 other integers to add up to the answer
    for i in (0..maximum_index).rev() {

        if entries[i] < half_sum {// Reached half from the right and found no answer so just quit
            break;
        } 

        if let Some(answer_pair) = find_sum_of_two(&entries[..i], 2020 - entries[i]){
            answer = answer_pair.0 * answer_pair.1 * entries[i];
            break;
        }
    }


    if answer==0 {println!("No answer found");} 
    else {println!("{}",answer)};
}
