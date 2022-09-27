fn main() {
    //ANCHOR: here
    let condition = true;
    let number = if condition { 5 } else { 6 }; //same return type

    // let number = if condition { 5 } else { "six" }; // incorrect

    println!("The value of number is: {number}");
    //ANCHOR_END: here
}
