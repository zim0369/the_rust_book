// ANCHOR: all
fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("Get first word");
    let word = first_word(&s);
    s.clear(); // Error!
    dbg!(word);
}
// ANCHOR: all
