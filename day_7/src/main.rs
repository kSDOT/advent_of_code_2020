//! Like previous
//! Changed the bitflag to make it so that the question must be answered yes from ALL in a single declaration 
//! (Previously it was so that the question was answered yes from ANY in a single declaration)
use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;
use std::pin::Pin;
#[derive(Eq, PartialEq, Clone)]
struct Bag {
    color: String
}

impl std::hash::Hash for Bag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state);
    }
}

impl Into<Bag> for String {
    fn into(self) -> Bag {
        Bag{color: self}
    }
}

//plaid aqua bags contain 4 shiny indigo bags, 2 pale teal bags, 3 clear crimson bags, 2 striped purple bags
fn main() {
    let input_file_path = "../day_7/src/input";
    
    let line_regex = Regex::new(r"^(?P<color>\w+ \w+) bags contain ((?P<others>(?:\d+) (.*))|(?:.*?))\.$").unwrap();
    let other_bags_regex = Regex::new(r"^(?P<count>\d+) (?P<color_name>\w+ \w+) bag(?:s)?(?:, )?$").unwrap();

    
    let mut all_bags: HashMap<Pin<Box<Bag>>, HashSet<&Bag>>= HashMap::new();

    let input_map =
        fs::read_to_string(input_file_path)
           .expect("Couldn't open file.")
           .lines()
           .map(|line|{

               let re = line_regex.captures(line).unwrap();
               let color: Bag = re["color"].to_owned().into();

               if let Some(other_bags) = &re.name("others") {
                   let mut bags = HashSet::new();
                    for capture in other_bags.as_str().split(", ") {
                        
                        let this_capture: String = other_bags_regex.captures(capture).unwrap()["color_name"].to_owned();
                        let box_bag = Pin::new(Box::new(this_capture.clone().into()));
                        all_bags.entry(box_bag).or_insert( HashSet::new());
                        
                        bags.insert(box_bag.as_ref());

                    }
                   //all_bags.insert(Box::new(color.into()), bags);

               }
               else {
                   //all_bags.insert(Box::new(color.into()), HashSet::new());
               }
             
               String::new()
            }
            ).collect::<Vec<String>>();
            

    //println!("{}", input_map.iter().fold(0, |acc, val| acc + val));
}
