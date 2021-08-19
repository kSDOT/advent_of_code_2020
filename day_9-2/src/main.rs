use std::fs;
use std::collections::{BTreeSet};

// returns whether the value can be expressed as a sum of 2 elements from the collection
fn get_sum(elements: &BTreeSet<i64>, value: i64) -> bool {
    // if we can
    for element in elements {
        if elements.get(&(value-element)).is_some() {
            return true;
        }
    }
    return false;
}

// returns the first number that doesn't obey the rules
fn find_first_number(numbers: &Vec<i64>) -> Option<i64> {
    // our current 'window' of elements we are working on, 25 elements wide
    let mut start = 0;
    let mut end = 25;
    // the elements btreeset facilitates the lookup
    let mut elements = numbers[start..end].iter().cloned().collect::<BTreeSet<i64>>();

    // we check for each element if they can be expressed as a sum of 2 elements from within the window
    for &value in &numbers[25..] {           
        if get_sum(&elements, value) == false {    // found the first element that can't be expressed as sum
            return Some(value);
        }

        // slide window 1 element to the right
        elements.remove(&numbers[start]);
        elements.insert(numbers[end]);

        start +=1;
        end +=1;
    }
    // no solution found
    return None;
}

// returns the min and max value of the range
fn find_range(numbers: &Vec<i64>, invalid_number: i64) -> (i64, i64){
    // can be improved by caching previous_sum

    // we sum up all elements until we find a range which sums up to the required number
    for (index, number) in numbers.iter().enumerate() {
        // keep sum for the current number
        let mut sum = *number;
        let mut new_index = index;

        // initiate min/max for this number
        let mut min = numbers[new_index];
        let mut max = min;
        new_index+=1;
        // sum up until we reach/exceed the required number
        while sum < invalid_number {
            let new_nr = numbers[new_index];
            // update min/max if applicable
            set_min_max(&mut min, &mut max, new_nr);
            // add to sum and continue to next nr
            sum += new_nr;
            new_index+=1;
        }
        // sum is equal to required number
        if sum == invalid_number {
            // so we update min/max (if min/max are the last element, the while loop will skip updating them)
            set_min_max(&mut min, &mut max, numbers[new_index]);
            return (min, max); // and return solution pair
        }

    }

    unreachable!(); // no solution to problem
}

// update min/max with the new value
fn set_min_max(min: &mut i64, max: &mut i64, value: i64) {
    if value < *min {
        *min = value;
    }
    else if value > *max {
        *max = value;
    }
}
fn main() {
    let input_file_path = "../day_9/src/input";
    let numbers: Vec<i64> = 
    fs::read_to_string(input_file_path)
        .expect("Couldn't open file.")
        .lines()
        .map(|line| // for each line we extract the command
            line.parse::<i64>().unwrap()
        )
        .collect();  
    
   let first_number = find_first_number(&numbers); // first number to break the rule

   let pair = find_range(&numbers, first_number.unwrap()); // find required range
   // solution is the sum of min+max value in the range
   println!("{:?}", pair.0 + pair.1);                                             
}

