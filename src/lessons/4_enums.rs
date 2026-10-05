#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    adddress: String,
}

pub(crate) fn enums() {
    println!("hello enums");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Four:: {:?} / Six:: {:?}", four, six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        adddress: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        adddress: String::from("::1"),
    };

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 10;
    let y: Option<i8> = Some(10);

    let sum = x + y;
}

fn route(ip_kind: IpAddrKind) {
    println!("Hello, IP Kind is: {:?}", ip_kind);
}
