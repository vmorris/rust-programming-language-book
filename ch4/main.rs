fn main() {
    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {y}");

    let a = [0; 1_000_000];
    let _b = a;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
