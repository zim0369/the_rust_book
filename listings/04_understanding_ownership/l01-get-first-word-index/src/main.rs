// ANCHOR: all
fn first_word(s: &String) -> usize {
    let s = s.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("Get first word");
    let word = first_word(&s);
    s.clear(); // word still has the value 5
    dbg!(word);
}
// ANCHOR: all
