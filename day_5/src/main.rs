//! Parse input from file into
//! Each line is interpreted as one ticket
//! Find the id for each ticket as we parse them and store in vector
//! Sort vector, last element will be the highest, which is what we require

use std::fs;
fn main() {
    let input_file_path = "../day_5/src/input";

    // BFFFBBFRRR
    // 1000110111
    //    1     0     0    0    1    1   0   1   1   1
    // (512) (256) (128) (64) (32) (16) (8) (4) (2) (1)
    //  (64)  (32)  (16)  (8)  (4)  (2) (1)|(4) (2) (1)


    // 'f','l' and 'b','r' are equivalent, the only difference is their position
    // the first 7 values are the higher bits

    let mut input_map =
        fs::read_to_string(input_file_path)
           .expect("Couldn't open file.")
           .lines()
           .map(|line|{ // for each line from input
                line.chars().fold(0, |acc, x| match x {
                    'F' | 'L' => (acc << 1) | 0,
                    'B' | 'R' => (acc << 1) | 1,
                    _ => panic!("Wrong input format!"),
                })
            }
            ).collect::<Vec<u32>>();

    input_map.sort();
    println!("{}", input_map.last().unwrap());
}
