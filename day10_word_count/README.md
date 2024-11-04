# Day 10: Word Count in a Sentence

## Objective
The goal for Day 10 was to create a program in Rust that counts the occurrences of each word in a user-provided sentence. The program handles case sensitivity by converting all words to lowercase and ignores punctuation to ensure accurate word counts.

## What I Learned
- How to use Rust's `HashMap` to store and manage key-value pairs.
- Splitting a sentence into words using `split_whitespace` and cleaning each word of punctuation.
- Using `entry` with `or_insert` in hash maps to update or insert values effectively.

## Program Flow
1. **Prompt for Input**:
    - The program asks the user to enter a sentence.
2. **Split and Clean Words**:
    - The input is split into words by whitespace, each word is cleaned of punctuation, and converted to lowercase.
3. **Count Word Occurrences**:
    - Each unique word is stored as a key in a hash map, with the count of occurrences as its value.
4. **Display Results**:
    - The program prints each word along with its count.

## Code Example

```rust
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
