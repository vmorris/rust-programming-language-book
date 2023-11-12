// Given a list of integers, use a vector and return
// the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        println!("\n-------\nInput numbers, one at a time.");
        println!("Enter a single period to signal end of input.");
        println!("Ctrl-C to exit");

        let mut numbers: Vec<i64> = Vec::new();
        let mut counts: HashMap<i64, i32> = HashMap::new();

        while let Some(number) = get_number() {
            numbers.push(number);
            let count = counts.entry(number).or_insert(1);
            *count += 1;
        }

        numbers.sort();
        println!("\nThe list of sorted numbers is {:?}", numbers);

        if numbers.len() == 0 {
            println!("No numbers entered, restarting program.");
            continue;
        }

        let median = numbers[numbers.len() / 2];
        println!("The median number is {:?}", median);

        let mut most = 0;
        let mut max = -1;
        for (key, val) in counts.iter() {
            if val > &max {
                most = *key;
                max = *val;
            };
        }
        println!("The number that occurs most often is {:?}", most);
    }
}

fn get_number() -> Option<i64> {
    let mut input_num = String::new();

    io::stdin()
        .read_line(&mut input_num)
        .expect("Failed to read line");

    if input_num == ".\n" {
        println!("End of input detected.");
        return None;
    }

    let input_num: i64 = match input_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid entry, restarting program.");
            return None;
        }
    };
    return Some(input_num);
}
