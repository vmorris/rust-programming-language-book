use console::Term;
use std::io;

fn main() {
    loop {
        let mut temp = get_temp();
        let mut unit = get_unit();
        println!("You entered: {temp} °{unit}");

        if unit == 'F' {
            // convert to Celsius
            unit = 'C';
            temp = (temp - 32.0) * (5.0 / 9.0);
        } else {
            // convert to Fahrenheit
            unit = 'F';
            temp = temp * (9.0 / 5.0) + 32.0;
        }
        println!("This equals {temp} °{unit}");
    }
}

fn get_temp() -> f64 {
    loop {
        println!("Input the temperature (Ctrl+c to exit):");
        let mut input_temp = String::new();

        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");

        let input_temp: f64 = match input_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return input_temp;
    }
}

fn get_unit() -> char {
    let term = Term::stdout();
    loop {
        println!("Input the unit ('F' for Fahrenheit, 'C' for Celsius):");

        let input_unit = match term.read_char() {
            Ok(val) => val,
            Err(_) => continue,
        };
        if ['f', 'F', 'c', 'C'].contains(&input_unit) {
            let input_unit = input_unit.to_ascii_uppercase();
            return input_unit;
        }
    }
}
