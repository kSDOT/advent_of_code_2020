//! Parse input from file and split it into separate declarations
//! Each line of declaration is the answer from one person
//! For each declaration we keep a bitflag that indicates which questions are answered yes
//! For each line(person) we set the bit corresponding to that question
use bitvec::prelude::*;
use std::fs;
fn main() {
    let input_file_path = "../day_6/src/input";

    // Questions go from a-z
    // Each position indicates if the question is answered yes(true) or not(false)
    let mut alphabet = bitvec![0; 26];

    let input_map =
        fs::read_to_string(input_file_path)
           .expect("Couldn't open file.")
           .split("\n\n") // Split into declarations
           .filter(|line| *line != "\n" && !line.is_empty())
           .map(|line|{ // for each person's answers
                alphabet.set_elements(0); // reset our bitflag
                
                line.chars()
                    .filter(|character| // remove empty lines, each character in a line must be 'a'-'z'
                         !character.is_whitespace()
                    )
                    .for_each(|character|
                         alphabet.set(character as usize - 'a' as usize, true)
                    );
                alphabet.iter().fold(0,|acc, val| acc + *val as u32) // count how many questions were answered with yes for this declaration
            }
            ).collect::<Vec<u32>>();

    println!("{}", input_map.iter().fold(0, |acc, val| acc + val));// count how many questions were answered with yes for all declarations
}
