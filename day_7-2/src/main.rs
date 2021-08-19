//! Like previous
//! Changed the way output is calculated because we require the number of bags reachable from our required bag

use regex::Regex;
use std::fs;
use std::collections::{HashMap};
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

    // contains all bags we discover and their count
    let mut all_bags: HashMap<Bag, Vec<(Bag, i64)>>= HashMap::new();    
       
    fs::read_to_string(input_file_path)
        .expect("Couldn't open file.")
        .lines()
        .for_each(|line|{// for each line we extract the outter bag

            let re = line_regex.captures(line).unwrap();
            let color: Bag = re["color"].to_owned().into();

            let mut bags = Vec::new();
            if let Some(other_bags) = &re.name("others") {

                for capture in other_bags.as_str().split(", ") {// and for each outter bag we find out all the contained bags
                        
                    let box_bag: (Bag, i64) = (other_bags_regex.captures(capture).unwrap()["color_name"].to_owned().into(),
                                                other_bags_regex.captures(capture).unwrap()["count"].parse::<i64>().unwrap());

                    all_bags.entry(box_bag.0.clone()).or_insert( Vec::new());
                        
                    bags.push(box_bag);

                }    

            }    
            all_bags.insert(color, bags); // insert a (outter bag, [(contained bag, count),..]) entry
   
        }   
        );    
    
    // call recursive function
    // the result of which will be the solution (-1, because the required bag itself should not be included in result)
    println!("{}", get_count(&all_bags, &required_bag.to_owned().into())-1);
}

fn get_count(all_that_reach: &HashMap<Bag, Vec<(Bag, i64)>>, current: &Bag) -> i64 {
    // for every bag contained in this one
    all_that_reach.get(&current)
                  .expect("invalid input")
                  .iter()
                  .fold(1, |accumulator, (bag, count)| 
                    // we get the number of bags the contained bag contains
                     count * get_count(&all_that_reach,bag) + accumulator 
                     // multiply how many of the contained bag we have by how many bags it contains, and add to the total
                   )
}