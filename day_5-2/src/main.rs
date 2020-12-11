//! Like previous 
//! But the solution is calculated differently because we require a different answer

use std::fs;
fn main() {
    let input_file_path = "../day_5/src/input";

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

    // first row will be 0 *8 + 0 --> 0  *8 + 7
    // last row will be 127*8 + 0 --> 127*8 + 7
    // if any of the id's between them are missing, thats our ticket id!
    // so if we have a missing id 
    input_map.sort();

    input_map.into_iter()
             .skip(8) //skip first row
             .collect::<Vec<_>>()
             .windows(2)  // for every two elements
             .any(|elements| 
                if elements[1] - 1  != elements[0]  { // if they do not have continuous  values, the value that would be in-between these two elements is our ticket
                    println!("{}", elements[1]-1); 
                    true    // when we find our ticket we return true to exit
                } 
                else { 
                    false 
                }
            ); // iteration doesn't check if we are counting the last row, because our ticket will have been found before we reach any element from the last row

}
