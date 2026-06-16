pub(crate) fn edit() {
    let mut text = String::from("hello rust");
    add_exclamation(&mut text);
    println!("{text}");

    clear_text(&mut text);
    println!("{text}");

    text_length(&text);
    println!("{text}");

    let r1 = &text;
    let r2 = &mut text;
    println!("{r1}");
    println!("{r2}");
}

fn add_exclamation(text: &mut String) {
    text.push_str("!");
    println!("added exclamation: {text}");
}

fn clear_text(text: &mut String) {
    text.clear();
    println!("Cleared text: {text}");
}

fn text_length(text: &String) {
    println!("Text length: {}", text.len());
}
