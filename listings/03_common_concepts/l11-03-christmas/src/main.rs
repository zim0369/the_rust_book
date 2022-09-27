//ANCHOR: all
fn main() {
    // 12
    let start = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    // 11
    let mid = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..=11 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            start[i]
        );
        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                print!("And ");
            }
            println!("{}", mid[j]);
        }
        println!();
    }
}
//ANCHOR_END: all
