fn main() {
    // ANCHOR: here
    let x = "four";

    {
        let x = x.len();
        println!("The length of string 'x' is: {x}");
    }

    println!("Five is greater than {x}");
    // ANCHOR_END: here
}
