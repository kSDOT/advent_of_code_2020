//! Parse input from file and split it into separate lines
//! Each line contains one number
//! We find the first number that breaks the rule
#![feature(map_first_last)]
#![feature(cell_update)]
use std::fs;
use std::collections::{BTreeSet};
use std::cell::Cell;
struct Node {
    joltage: u64, 
    splits: Cell<u64>,
    accumulated_path: Cell<u64>
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Joltage: {}\t Splits: {}\t Acc: {}", self.joltage, self.splits.get(), self.accumulated_path.get())
    }
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Joltage: {}\t Splits: {}\t Acc: {}", self.joltage, self.splits.get(), self.accumulated_path.get())
    }
}
fn main() {
    let input_file_path = "../day_10/src/input";

    let mut jolts: BTreeSet<u64> = 
    fs::read_to_string(input_file_path)
        .expect("Couldn't open file.")
        .lines() 
        .map(|line| // for each line we extract the command
            line.parse::<u64>().unwrap()
        )
        .collect();
    //jolts.insert(0);
    //jolts.insert(jolts.last().unwrap()+3);
   let mut jolts = jolts.into_iter().rev().map(|x| 
                                                        Node{joltage: x, splits: Cell::new(1), accumulated_path: Cell::new(0)}
                                                        ).collect::<Vec<Node>>();

   let result = helper(&mut jolts);
        println!("------\n{:?}",jolts);
   println!(""); 
}

fn helper(jolts: &mut Vec<Node>) -> u64 {
   let mut acc = 1; 
    jolts.as_mut_slice()
         .windows(4)
         .into_iter()
         .for_each(|values|{
            
             println!("{:?}", values);
             if values[0].joltage - values[2].joltage <= 3 {
                 values[2].splits.update(|x| x+1);
                 values[2].accumulated_path.update(|x| x+values[0].accumulated_path.get());
                if values[0].joltage - values[3].joltage <=3 {
                    values[3].splits.update(|x| x+1);
                    values[3].accumulated_path.update(|x| x+values[0].accumulated_path.get());


                }
             }

             let temp= values[0].splits.get();
             acc+= temp;
         } );
     return acc-1;
 }