fn main() {
    days_of_christmas();
}

fn days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelveth",
    ];
    let gift = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let mut index = 0;
    while index < 12 {
        println!(
            "On the {} day of Christmas my true love sent to me,",
            days[index]
        );
        let mut inner_index = index;
        while inner_index > 0 {
            println!("{},", gift[inner_index]);
            inner_index -= 1;
        }
        if index != 0 {
            print!("and ");
        }
        println!("{}", gift[inner_index]);
        println!("");
        index += 1;
    }
}
