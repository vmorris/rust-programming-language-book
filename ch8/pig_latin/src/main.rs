// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to
// the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    loop {
        println!("Enter a string:");
        println!("Ctrl-C to exit");

        let input = get_string();
        let input_words: Vec<String> = input.split_whitespace().map(str::to_string).collect();
        let mut output_words: Vec<String> = Vec::new();

        let vowels = "aeiouAEIOU";

        for word in input_words {
            let mut word_chars = word.chars();
            let new_word;
            let first_char = word_chars.next().unwrap();
            let remaining_word = word_chars.collect::<String>();

            if vowels.contains(first_char) {
                new_word = format!("{word}-hay");
            } else {
                new_word = format!("{remaining_word}-{first_char}ay");
            }

            output_words.push(new_word);
        }

        println!("{}", output_words.join(" "));
    }
}

fn get_string() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim_end().to_string();
}
