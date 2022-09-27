// TODO: complete code

// ANCHOR: all
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    print!(
        "First word is: {}",
        first_word(&String::from("First word is:"))
    )
}
// ANCHOR_END: all
