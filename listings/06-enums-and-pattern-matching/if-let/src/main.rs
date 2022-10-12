fn main() {
    // ANCHOR: all 
    let cash = Some(69);
    if let Some(69) = cash {
        println!("Yay!");
    } else {
        println!("Missed it!");
    }
    // ANCHOR: all 
}
