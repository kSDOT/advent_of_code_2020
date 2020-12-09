//! We parse all the entries and place them in a vector
//! For each entry we test if it's valid and count it

use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;
use std::ops::RangeInclusive;
// Structure that will hold the parsed data
#[derive(Debug)]
struct Policy {
    password: String,
    letter: char,
    count: (u32, u32),
}

// Data parsing
fn string_to_policy(input: std::result::Result<std::string::String, std::io::Error>) -> Policy {
    let mut input = input.unwrap();
    let mut input = &mut input.chars();

    let left = input.take_while(|c|c.is_digit(10))
                                .fold(None, |acc, ch| {
                                    ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
                                })
                                .unwrap();
    

    let right = input.take_while(|c|c.is_digit(10))
                                .fold(None, |acc, ch| {
                                    ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
                                })
                                .unwrap();

    let count = (left, right);

    let letter = input.next().unwrap();

    let password = input.skip(2).collect();

    Policy{
        password,
        letter,
        count,
    }
}

// Check if policy is valid
impl Policy {
    fn is_valid(&self) -> bool {
        // The letter should be EITHER in the first position or the second, but not in both
        // So we xor both positions
        let mut temp = self.password.chars();
        println!("{} -> {:?}", self.password, self.count);
        (temp.nth((self.count.0 -1) as usize).unwrap() == self.letter) ^ 
        (temp.nth((self.count.1 -1 - self.count.0) as usize).unwrap() == self.letter)
    }
}
fn main() {
    let input_file_path = "../day_2/src/input";
    // Read each line from file and parse it
    let mut entries = BufReader::new(
        File::open(input_file_path).expect("Couldn't open file.")
            )
            .lines()
            .map(string_to_policy)
            .collect::<Vec<Policy>>();
    // Count the valid entries
    println!("{}", entries.into_iter().fold(0,|acc, element| acc + (element.is_valid() as u32)));
}
