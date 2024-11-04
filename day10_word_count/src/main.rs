use std::io;
use std::collections::HashMap;

fn main() {
    let sentence = get_sentence();
    let word_count = count_words(&sentence);
    display_word_counts(&word_count);
}

fn get_sentence() -> String {
    println!("Enter a sentence:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn count_words(sentence: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();

    for word in sentence.split_whitespace() {
        let word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
}

fn display_word_counts(word_count: &HashMap<String, u32>) {
    println!("Word counts:");
    for (word, count) in word_count {
        println!("{}: {}", word, count);
    }
}