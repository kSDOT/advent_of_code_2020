//! Like previous solution, but modified data validity check
use std::fs;
use regex::Regex;
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Unit,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Unit::no_value,
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: String::new(),
        }
    }
}
#[derive(PartialEq, Eq)]
enum Unit {
    cm(u32),
    inch(u32),
    no_unit(u32),
    no_value
}

fn main() {
    let input_file_path = "../day_4/src/input";

    let input_map =
        fs::read_to_string(input_file_path)
           .expect("Couldn't open file.")
           .split("\n\n")
           .map(|line|{
                let mut passport = Passport::new();
                line.split([' ', '\n'].as_ref()).for_each(|field_value| parse_field(&mut passport, field_value));
                passport
            }
            ).collect::<Vec<Passport>>();

    println!("{}", input_map.into_iter().filter(is_valid).count());
}

fn parse_field(passport: &mut Passport, field_value: &str){
    let mut field_value = field_value.split(':');

    if let Some(field) = field_value.next() {
        if let Some(value) = field_value.next() {
    
        match field {
            "byr" => passport.byr = Some(value.parse().expect("Error parsing data: Wrong format of input data!")),
            "iyr" => passport.iyr = Some(value.parse().expect("Error parsing data: Wrong format of input data!")),
            "eyr" => passport.eyr = Some(value.parse().expect("Error parsing data: Wrong format of input data!")),
            "hgt" => {
                passport.hgt = 
                    if value.ends_with(|last_char: char| last_char.is_digit(10)){ Unit::no_unit(value.parse().expect("Error parsing data: Wrong format of input data!") )} 
                    else if value.ends_with("in") {Unit::inch(value[..value.len()-2].parse().expect("Error parsing data: Wrong format of input data!"))} 
                    else if value.ends_with("cm") {Unit::cm(value[..value.len()-2].parse().expect("Error parsing data: Wrong format of input data!"))}
                    else {panic!("Error parsing data: Wrong format of input data!")}       
               }, 
            "hcl" => passport.hcl = value.to_owned(),
            "ecl" => passport.ecl = value.to_owned(),
            "pid" => passport.pid = value.to_owned(),
            "cid" => passport.cid = value.to_owned(),
            _ => panic!("Error parsing data: Field is invalid"),
            }
        }
    }
}

fn is_valid<'a>(passport: &'a Passport) -> bool {
    passport.byr.is_some() && (1920..2003).contains(&passport.byr.unwrap()) &&
    passport.iyr.is_some() && (2010..2021).contains(&passport.iyr.unwrap()) &&
    passport.eyr.is_some() && (2020..2031).contains(&passport.eyr.unwrap()) &&
    (
    if let Unit::cm(value) = passport.hgt { (150..194).contains(&value) } else {false}|| 
    if let Unit::inch(value) = passport.hgt { (59..77).contains(&value) } else {false}
    ) &&
    Regex::new(r"^#(\d|[a-f]){6}$").unwrap().is_match(&passport.hcl) &&
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&(passport.ecl.as_ref())) &&
    Regex::new(r"^\d{9}$").unwrap().is_match(&passport.pid)
}