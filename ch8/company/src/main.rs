// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

type Company = HashMap<String, Vec<String>>;

fn main() {
    println!("Commands start with 'Add' or 'Show'");
    println!("\nExamples:");
    println!("Add Sally to Engineering");
    println!("or");
    println!("Add Amir to Sales");
    println!("or");
    println!("List Engineering");
    println!("or");
    println!("List all");

    let mut company: Company = HashMap::new();

    loop {
        println!("\n------\nEnter a command, ctrl-C to exit.");
        let input = get_string();
        let input_words: Vec<String> = input.split_whitespace().map(str::to_string).collect();

        let command_word = input_words[0].clone();

        match command_word.as_str() {
            "Add" => add_command(input_words, &mut company),
            "List" => list_command(input_words, &mut company),
            &_ => println!("Invalid command found: {command_word}"),
        }
    }
}

fn get_string() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim_end().to_string();
}

fn add_command(input_words: Vec<String>, company: &mut Company) {
    if input_words.len() != 4 || input_words[2] != "to" {
        println!("Invalid Add command detected.");
        return;
    }
    let employee = input_words[1].clone();
    let department = input_words[3].clone();
    company
        .entry(department)
        .and_modify(|employees| employees.push(employee.clone()))
        .or_insert(vec![employee]);
}

fn list_command(input_words: Vec<String>, company: &mut Company) {
    if input_words.len() != 2 {
        println!("Invalid List command detected.");
        return;
    }
    let target = input_words[1].clone();
    if target == "all" {
        println!("{:?}", company);
    } else {
        match company.get(&target) {
            Some(department) => {
                println!("{:?}", department);
            }
            None => println!("Couldn't find a department with that name!"),
        }
    }
}
