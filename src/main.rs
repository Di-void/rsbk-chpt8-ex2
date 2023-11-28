// Convert stings to pig latin

// The first consonant in each word is moved to the end of the word
// and "ay" is added. E.g "first" becomes "irst-fay"
// Words that start with a vowel have "hay" added to the end instead
// ("apple" becomes "apple-hay")
// Keep in mind the details about UTF-8 encoding

use std::io;

fn main() {
    let mut buffer = String::new();

    println!("Enter a string");
    
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let mut buffer = buffer.trim().to_string();

    // assuming we receive only one word
    if let Some(c) = buffer.chars().next() {
        let is_consonant = is_consonant(&c.to_string().to_lowercase());
        if is_consonant {
            let c = buffer.remove(0);
            let c = format!("-{}ay", c);
            buffer.push_str(&c);
        } else {
            buffer.push_str("-hay");
        }
    };
    
    println!("{}", buffer);
}

fn is_consonant (str: &str) -> bool {
    let vowels = vec!["a", "e", "i", "o", "u"];

    for v in vowels {
        if v == str {
            return false
        }
    }

    true
}