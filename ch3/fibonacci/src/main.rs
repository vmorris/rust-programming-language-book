use std::io;

fn main() {
    loop {
        let n = get_input();
        if n < 0 {
            println!("Input must be >= 0.");
            continue;
        }
        let result = fibonacci(n);
        println!("The {n} fibonacci number is {result}");
    }
}

// recursive implementation
// this would be better if we had caching into a lookup table
// to avoid repeated calculations of the same result.
fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn get_input() -> i64 {
    loop {
        println!("Input 'n' for which Fibonacci number to generate, Ctrl-C to exit:");
        let mut input_n = String::new();

        io::stdin()
            .read_line(&mut input_n)
            .expect("Failed to read line");

        let input_n: i64 = match input_n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return input_n;
    }
}
