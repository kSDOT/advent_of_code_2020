//! Parse input from file and split it into separate lines
//! Each line contains an outter bag and any other bag inside of it
//! Once we know which color bag contains what other colors, we find out how many contain the gold bag

use regex::Regex;
use std::fs;
use std::collections::{HashSet, HashMap};
//Strongly type our Bag
#[derive(Eq, PartialEq, Hash, Clone)]
struct Bag (String);

impl From<String> for Bag {
    fn from(s: String) -> Self {
        Bag(s)
    }
}

pub fn main() {
    let input_file_path = "../day_7/src/input";
    let required_bag = "shiny gold";
    // extracts name of outter bag from a line
    let line_regex = Regex::new(r"^(?P<color>\w+ \w+) bags contain ((?P<others>(?:\d+) (.*))|(?:.*?))\.$").unwrap();

    // extracts the name and count of each contained bag 
    let other_bags_regex = Regex::new(r"^(?P<count>\d+) (?P<color_name>\w+ \w+) bag(?:s)?(?:, )?$").unwrap();

    // contains all bags we discover
    let mut all_bags: HashMap<Bag, Vec<Bag>>= HashMap::new();    
       
    fs::read_to_string(input_file_path)
        .expect("Couldn't open file.")
        .lines()
        .for_each(|line|{// for each line we extract the outter bag

            let re = line_regex.captures(line).unwrap();
            let color: Bag = re["color"].to_owned().into();

            let mut bags = Vec::new();
            if let Some(other_bags) = &re.name("others") {

                for capture in other_bags.as_str().split(", ") {// and for each outter bag we find out all the contained bags
                        
                    let box_bag: Bag = other_bags_regex.captures(capture).unwrap()["color_name"].to_owned().into();

                    all_bags.entry(box_bag.clone()).or_insert( Vec::new());
                        
                    bags.push(box_bag);

                }    

            }    
            all_bags.insert(color, bags);// insert a (outter bag, [contained bag,..]) entry
   
        }   
        );    
                

    let mut all_that_reach = HashSet::new();
    let mut continue_iteration = true;
    // for each of the outter bags we have in our lookup table
    // in first iteration (level 0): we check if their contained bag contains the required bag
    // in the following iterations (level 1): we check if the contained bags contain any bag that contains the required bag
    // in last iteration (level x): we have gone x levels up, and on this iteration no new candidate was found, so we exit loop 
    while continue_iteration {
        continue_iteration = false;
        all_bags.iter().for_each(|(container_bag, other_bags)|{
            if !all_that_reach.contains(container_bag){ 
                for bag in other_bags {
                    if bag.0 == required_bag || all_that_reach.contains(bag) {
                        all_that_reach.insert(container_bag);// bags that directly or indirectly contain the required one are inserted
                        continue_iteration = true;
                        continue;
                }
            }}
        });
    }

    // the total number of all bags that directly or indirectly reach
    println!("{}", all_that_reach.len());
}

