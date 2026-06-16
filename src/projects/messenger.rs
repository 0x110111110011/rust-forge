pub(crate) fn send_message() {
    let message = String::from("hello");
    read_message(&message);
    send_and_return(message);
    send(message);
}

fn send(msg: String) {
    println!("Message Sent: {msg}");
}

fn send_and_return(msg: String) -> String {
    println!("Message Sent: {msg}");
    println!("Returned it: {msg}");
    msg
}

fn read_message(msg: &String) {
    println!("Reading message: {msg}");
}
