//! Like previous
//! Changed the bitflag to make it so that the question must be answered yes from ALL in a single declaration 
//! (Previously it was so that the question was answered yes from ANY in a single declaration)
use bitvec::prelude::*;
use std::fs;
fn main() {
    let input_file_path = "../day_6/src/input";

    let mut alphabet = bitvec![1; 26];

    let mut input_map =
        fs::read_to_string(input_file_path)
           .expect("Couldn't open file.")
           .split("\n\n")
           .filter(|line| *line != "\n" && !line.is_empty())
           .map(|declaration|{
                alphabet.set_all(true); // reset our bitflag
                // if someone hasn't answered yes to any question, we set that answer to false
                // answers set to false will remain that way for the rest of declaration
                declaration.lines().for_each(|line|alphabet.for_each(|index, bit| bit & line.contains((index as u8 + 'a' as u8) as char)));
                
                alphabet.iter().fold(0,|acc, val| acc + *val as u32) // count how many were true in the end
            }
            ).collect::<Vec<u32>>();

    println!("{}", input_map.iter().fold(0, |acc, val| acc + val));
}
