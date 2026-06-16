pub(crate) fn find() {
    let sentence = String::from("Rust ownership makes memory safety possible");
    let rust = first_name(&sentence);
    println!("First word: {rust}");

    let ownership = second_word(&sentence);
    println!("Second word: {ownership}");

    let longest = longest_word(&sentence);
    println!("Longest word: {longest}");
}

fn first_name(text: &str) -> &str {
    match text.split_whitespace().next() {
        Some(word) => word,
        None => text,
    }

    // OR
    // text.split_whitespace().next().unwrap_or(text)

    // OR
    // for (i, word) in text.split_whitespace().enumerate() {
    //     if i == 0 {
    //         return word;
    //     }
    // }
    //
    // text
}

fn second_word(text: &str) -> &str {
    text.split_whitespace().nth(1).unwrap_or(text)
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or(text)

    // OR
    // let mut longest = "";
    // for word in text.split_whitespace() {
    //     if word.len() > longest.len() {
    //         longest = word;
    //     }
    // }
    // longest
}
