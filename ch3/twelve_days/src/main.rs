fn main() {
    let lyrics = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "three french hens"),
        ("fourth", "four calling birds"),
        ("fifth", "five golden rings"),
        ("sixth", "six geese a-laying"),
        ("seventh", "seven swans a-swimming"),
        ("eighth", "eight maids a-milking"),
        ("nineth", "swans a-swimming"),
        ("tenth", "ten lords a-leaping"),
        ("eleventh", "eleven pipers piping"),
        ("twelfth", "twelve drummers drumming"),
    ];

    for i in 0..12 {
        let a = lyrics[i].0;
        println!("On the {a} day of Christmas, my true love gave to me");
        for j in (0..=i).rev() {
            let b = lyrics[j].1;
            if i > 0 && j == 0 {
                println!("and a {b}");
            } else {
                println!("{b}");
            }
        }
    }
}
