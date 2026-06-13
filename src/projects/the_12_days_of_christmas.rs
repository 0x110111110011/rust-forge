pub(crate) fn christmas_present() {
    const DAYS_OF_CHRISTMAS: [&str; 12] = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eight", "Nine", "Ten",
        "Eleven", "Twelve",
    ];

    const GIFTS_OF_CHRISTMAS: [&str; 12] = [
        "GIFT 01", "GIFT 02", "GIFT 03", "GIFT 04", "GIFT 05", "GIFT 06", "GIFT 07", "GIFT 08",
        "GIFT 09", "GIFT 10", "GIFT 11", "GIFT 12",
    ];

    println!("The Twelve Days Of Christmas.\n");

    for (day_index, day) in DAYS_OF_CHRISTMAS.iter().enumerate() {
        println!("On the {day} day of Christmas my true love sent to me");

        for gift_index in (0..=day_index).rev() {
            if day_index > 0 && gift_index == 0 {
                println!("And a {}", GIFTS_OF_CHRISTMAS[gift_index]);
                break;
            }
            println!("a {}", GIFTS_OF_CHRISTMAS[gift_index]);
        }

        println!();
    }
}
