//! Parse input from file and split it into separate lines
//! Each line contains one command and its value
//! After we have all the commands, we 'execute' them in our Virtual Machine (struct Commands)
//! When we encounter a command we have already executed(meaning we are stuck in a loop):
//! 1- We save the virtual call stack (struct HistoryStack)
//! 2- We make a change to the latest NOP/JMP and restart execution from that snapshot
//! 3- Continue until we finish executing all commands
//! 4- If we get stuck in a loop again, it means the instruction we swapped wasn't the faulty one,
//!    so we revert the change and try an earlier snapshot

use std::fs;
use std::str::FromStr;
use std::collections::{HashSet};
// Helper struct, saves a snapshot at a current moment of the Virtual Machine Execution
#[derive(Debug)]
struct HistoryStack {
    //(Command, Index of Command, Accumulator value)
    stack: Vec<(Command, usize, i32)>, // save the state of our virtual machine at this instruction
    problem_index_limit: usize,        // save index of the element where the error first occurred, the problem lies between [0, problem_index_limit]
    modified: bool,                    // rembember if this struct was modified
}

// Our Virtual Machine, contains Stack, Stackpointer and an Accumulator
// Also contains a HistoryStack with snapshots of our VM
#[derive(Debug)]
struct Commands {
    stackpointer: usize,         // which instruction we are at
    commands: Vec<Command>,      // all instructions
    acc: i32,                    // current value
    history_stack: HistoryStack, // save all nop/jmp values we have executed 
}

impl Commands {

    fn new(commands: Vec<Command>) -> Commands {
        Commands {
            stackpointer: 0,
            commands,
            acc: 0,
            history_stack:  HistoryStack{
                stack: Vec::new(),
                problem_index_limit: 0,
                modified: false,
            },
        }
    }

    fn execute(&mut self, seen_values: &mut HashSet<usize>) {

        loop {
            if  seen_values.insert(self.stackpointer) == false {   // we found a loop, enter restore phase
                // we will restart execution, it's not necessary to snapshot/restore
                seen_values.clear();                               

                // remove last instruction on stack history, also restores previous attempted change
                let temp = self.unwind_last();  
                if let Some(pair) = temp {             // removing was valid so
                    self.modify_last(pair);         // we modify the next instruction
                } 
            }   
            else {
                // succesfully finished executing all commands so we exit the virtual machine
                if self.stackpointer >= self.commands.len(){ return;}  
                
                // get next command and execute it
                match self.commands[self.stackpointer] {  
                    Command::Acc(x) => {                  
                        self.stackpointer += 1; 
                        self.acc += x; 
                    },
                    Command::Jmp(x) => {
                        // stores the current JMP/NOP context of execution
                        self.history_stack.stack.push((Command::Jmp(x), self.stackpointer, self.acc));
                        self.stackpointer = (self.stackpointer as i32 + 1 * x) as usize;
                    },
                    Command::Nop(x) => {
                        // stores the current JMP/NOP context of execution
                        self.history_stack.stack.push((Command::Nop(x), self.stackpointer, self.acc));
                        self.stackpointer += 1;
                    },
                }
            }
        }
    }
    
    // swaps the current instruction NOP <=> JMP
    fn swap_nop_jmp(&mut self, stackpointer: usize) {
        self.commands[stackpointer] = match self.commands[stackpointer]{
            Command::Jmp(x) => Command::Nop(x),
            Command::Nop(x) => Command::Jmp(x),
            Command::Acc(x) => unreachable!(),// stackpoint will point to either JMP/NOP so this will never be reached
                                                  // if it is reached the logic is flawed and the program should terminate
        };
    }

    // Pop the latest value from History Stack
    // Returns the popped value containing (stackpointer, accumulator)
    fn unwind_last(&mut self) -> Option<(usize, i32)> {
        if !self.history_stack.modified { // first unwind, no change to restore
            self.history_stack.modified = true;
            self.history_stack.problem_index_limit = self.history_stack.stack.len()-1;
        }
        else {// restore the previous failed change 
            self.swap_nop_jmp(self.history_stack.stack[self.history_stack.problem_index_limit].1);
            self.history_stack.problem_index_limit-=1;
        }
        // snapshot of stackpointer/accumulator at this command
        return self.history_stack.stack.get(self.history_stack.problem_index_limit)
                                       .map(|tuple| (tuple.1, tuple.2));
    }

    // Restores the VM to the provided snapshopt
    // Swap the latest value NOP<=>JMP
    fn modify_last(&mut self, pointer_acc_pair: (usize, i32)) {
        // restore context

        self.stackpointer = pointer_acc_pair.0;
        self.acc = pointer_acc_pair.1;
        self.swap_nop_jmp(self.stackpointer);
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

