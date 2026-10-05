pub(crate) fn practice() {
    println!("Struct Practice.");
    let width = 30;
    let height = 50;
    println!(
        "The area of the ractangle is: {} square pixels.",
        area(width, height)
    );
}
fn refactor_with_tuple() {
    let rectangle = (30, 50);
    println!(
        "The area of the ractangle with tuple: {}",
        area_with_tuple(rectangle)
    );
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn refactor_with_struct() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of rectangle with struct: {:?}",
        area_with_struct(&rectangle)
    );
    println!("OR : {rectangle:#?}");
}
fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn struct_with_method() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle with method: {}",
        rectangle.area()
    );
}
