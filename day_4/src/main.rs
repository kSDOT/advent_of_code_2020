//! Parse input from file into our struct for each passport
//! Check validity

use std::fs;

struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<u32>,
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
            hgt: None,
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: String::new(),
        }
    }
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
    // data format 
    // field:value
    let mut field_value = field_value.split(':');

    if let Some(field) = field_value.next() {
        if let Some(value) = field_value.next() {
    
        match field {
            "byr" => passport.byr = Some(value.parse().expect("Error parsing data: Wrong format of input data!")),
            "iyr" => passport.iyr = Some(value.parse().expect("Error parsing data: Wrong format of input data!")),
            "eyr" => passport.eyr = Some(value.parse().expect("Error parsing data: Wrong format of input data!")),
            "hgt" => passport.hgt = Some(
                (if value.ends_with(|last_char: char| last_char.is_digit(10)){ value} else { &value[..value.len()-2] })
                .parse()
                .expect("Error parsing data: Wrong format of input data!")
            ),  
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
    passport.byr.is_some() &&
    passport.iyr.is_some() &&
    passport.eyr.is_some() &&
    passport.hgt.is_some() &&
    !passport.hcl.is_empty() &&
    !passport.ecl.is_empty() &&
    !passport.pid.is_empty() 
}