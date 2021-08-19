//! Parse input from file and split it into separate lines
//! Each line contains one command and its value
//! After we have all the commands, we 'execute' them in our Virtual Machine (struct Commands)
//! When we encounter a command we have already executed we finish execution,
//! Because we are stuck in a loop

use std::fs;
use std::str::FromStr;
use std::collections::{HashSet};
#[derive(Debug)]
struct Commands {
    stackpointer: usize,         // which instruction we are at
    commands: Vec<Command>,      // all instructions
    acc: i32,                    // current value
}

impl Commands {
    fn new(commands: Vec<Command>) -> Commands {
        Commands {
            stackpointer: 0,
            commands,
            acc: 0,
        }
    }
    fn execute(&mut self, seen_values: &mut HashSet<usize>) {

        loop {
            if  seen_values.insert(self.stackpointer) == false { // we found a loop
                return;                                          // return, the solution is held in self.acc
            }   
            else {
                // finished executing all commands (sanity check in case no solution)  
                if self.stackpointer >= self.commands.len(){ break; }   
                
                // get next command and execute it
                match self.commands[self.stackpointer] {                
                    Command::Acc(x) => {self.stackpointer += 1; self.acc += x; },
                    Command::Jmp(x) => {self.stackpointer = (self.stackpointer as i32 + 1 * x) as usize;},
                    Command::Nop(_) => {self.stackpointer += 1},
                }
            }
        }
    }
    
}
#[derive(Debug, PartialEq, Eq)]
enum Command {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
// parsing input
impl std::str::FromStr for Command {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..3] {
            "acc" => Ok(Command::Acc(s[4..].parse().unwrap())),
            "jmp" => Ok(Command::Jmp(s[4..].parse().unwrap())),
            "nop" => Ok(Command::Nop(s[4..].parse().unwrap())),
             _ => panic!("wrong input"),
        }
    }
}

fn main() {
    let input_file_path = "../day_8/src/input";
    let commands = 
    fs::read_to_string(input_file_path)
        .expect("Couldn't open file.")
        .lines()
        .map(|line| // for each line we extract the command
            Command::from_str(line).unwrap()
        )
        .collect();  
    let mut commands = Commands::new(commands);  // we build our commands structure
    let mut seen_commands = HashSet::new(); // position of all executed commands
    
    commands.execute(&mut seen_commands);    // execute our VM
    println!("{}",commands.acc);                        // acc value when we are finished executing
}

