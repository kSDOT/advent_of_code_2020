//! Parse input from file and split it into separate lines
//! Each line contains one number
//! We find the first number that breaks the rule

use std::fs;
use std::collections::{BTreeSet};

// returns whether the value can be expressed as a sum of 2 elements from the collection
fn get_sum(elements: &BTreeSet<i64>, value: i64) -> bool {
    // if we can
    for element in elements {
        if elements.get(&(value-element)).is_some() {
            return true;
        }
    }
    return false;
}

// returns the first number that doesn't obey the rules
fn find_first_number(numbers: &Vec<i64>) -> Option<i64> {
    // our current 'window' of elements we are working on, 25 elements wide
    let mut start = 0;
    let mut end = 25;
    // the elements btreeset facilitates the lookup
    let mut elements = numbers[start..end].iter().cloned().collect::<BTreeSet<i64>>();

    // we check for each element if they can be expressed as a sum of 2 elements from within the window
    for &value in &numbers[25..] {           
        if get_sum(&elements, value) == false {    // found the first element that can't be expressed as sum
            return Some(value);
        }

        // slide window 1 element to the right
        elements.remove(&numbers[start]);
        elements.insert(numbers[end]);

        start +=1;
        end +=1;
    }
    // no solution found
    return None;
}

fn main() {
    let input_file_path = "../day_9/src/input";
    let numbers: Vec<i64> = 
    fs::read_to_string(input_file_path)
        .expect("Couldn't open file.")
        .lines()
        .map(|line| // for each line we extract the command
            line.parse::<i64>().unwrap()
        )
        .collect();  
    
   let first_number = find_first_number(&numbers); // first number to break the rule
             
   println!("{}", first_number.unwrap()); 
}

