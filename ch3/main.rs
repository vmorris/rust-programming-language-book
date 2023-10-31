fn main() {
    let mut power: f = 1.0;
    let mut b: f32 = 1.0;
    while b != 0.0 {
        power = power / 2.0;
        let a: f32 = 1.0 + power;
        b = a - 1.0;
        println!("{power}\t{a}\t{b}")
    }
    let x: f32 = 2.0 * power;
    println!("Unit round = {x}");
}
