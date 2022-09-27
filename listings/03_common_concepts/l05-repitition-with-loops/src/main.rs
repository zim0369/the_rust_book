fn main() {
    //ANCHOR: here
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // breaks loop and returns a value
        }
    };

    println!("The result is {result}");
    //ANCHOR_END: here
}
