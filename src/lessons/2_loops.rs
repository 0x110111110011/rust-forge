pub(crate) fn loops() {
    let mut counter = 0;

    'upper_loop: loop {
        println!("Counter: {counter}");
        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'upper_loop;
            }
            remaining -= 1;
        }
        counter += 1;
    }

    println!("Counter ended: {counter}");
}
