pub(crate) fn variable() {
    // 1. mutable / immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 24;
    println!("The next value of x is: {x}");

    // 2. shadowing variable
    let shadow = 5;
    let shadow = shadow + 1;
    {
        let shadow = shadow + 2;
        println!("Shadowing from inner scope: {shadow}");
    }
    println!("Shadowing from upper scope: {shadow}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

    /*
     * NOTE: - Integer overflow
     * panic! in debug mode.
     * complement wrapping in release mode(which means values greater than the maximum value the type can hold “wrap around”
     * to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1,
     * and so on.)
     * */
}
