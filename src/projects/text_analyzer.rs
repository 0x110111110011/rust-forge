pub(crate) fn analyze() {
    println!("Text analyzer.");
    // let text = String::from("rust is fast and rust is safe");
    // let count = count_word(&text, "rust");
    // println!("Count: {count}");

    analyzer();
}

fn count_word(text: &str, target: &str) -> usize {
    text.split_whitespace().filter(|&w| w.eq(target)).count()
}

fn analyzer() {
    // 1. The Setup
    let text = String::from(
        "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. The borrow checker is your best friend, even if it feels like a strict teacher at first. Ownership, borrowing, and slices form the core of Rust memory management. Do not fight the strict rules. Learn to understand the borrow checker, and your code will be fast and safe.",
    );

    // 2. Find the Longest Word
    let longest = find_longest_word(&text);
    println!("The longest word is: '{}'", longest);

    // 3. Word Frequency
    let target = "borrow";
    let count = word_frequency(&text, target);
    println!("The word '{}' appears {} times.", target, count);

    // 4. Censorship (Mutation and Ownership)
    // We want to censor the word "strict"
    let censored_text = censor_text(text);
    println!("Censored text:\n{}", censored_text);

    // 5. The Borrow Checker Test
    // TODO: Try to print the original `text` variable here.
    // Figure out why it fails and how to fix it based on Chapter 4 rules!
    // println!("Original text:\n{}", text);
}

fn find_longest_word(text: &str) -> &str {
    let mut longest_word = "";

    for word in text.split_whitespace() {
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }

    longest_word
}

fn word_frequency(text: &str, target: &str) -> u32 {
    let mut count = 0;
    for word in text.split_whitespace() {
        if target == word {
            count = count + 1;
        }
    }
    count
}

fn censor_text(text: String) -> String {
    let cencor = text.replace("strict", "***");
    cencor
}
