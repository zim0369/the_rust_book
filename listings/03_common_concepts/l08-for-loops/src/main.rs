fn main() {
    //ANCHOR: here
    for number in (1..4).chain((1..4).rev()) {
        print!("{number} ");
    }
    //ANCHOR_END: here
}
