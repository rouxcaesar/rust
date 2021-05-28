fn main() {
    let lyrics = [
        "a partridge in a pear tree!",
        "two turtle doves and",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    let mut days: i8 = 1; 

    while days < 13 {
        let mut suffix: &str = "th";

        if days == 1 {
            suffix = "st";
        } else if days == 2 {
            suffix = "nd";
        } else if days == 3 {
            suffix = "rd";
        }

        println!("On the {}{} day of Christmas, my true love gave to me:", days, suffix);

        for lyric in  (0..days).rev() {
            println!("{}", lyrics[lyric as usize]);
        }

        println!("");
        days += 1;
    }
}
