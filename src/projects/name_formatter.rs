pub(crate) fn format() {
    println!("Name Formatter.");
    let mut name = String::from("wai hlyan phyo");
    // Printing name
    print_name(&name);

    // Capitalize name
    let upper = capitalize_name(&mut name);
    print_name(&upper);

    // Consume name
    consume_name(name);
    // print_name(&name);// ERROR
}

fn print_name(name: &str) {
    println!("{name}");
}
fn capitalize_name(name: &mut String) -> String {
    name.to_uppercase()
}
fn consume_name(name: String) {
    println!("Consumed name: {name}");
}
