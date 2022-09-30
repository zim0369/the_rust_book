// ANCHOR: all

// We use tuple structs when we want to give names to tuples
struct Scores(i32, f32, u32);

struct User {
    _name: String,  // Can't use "str" here
    scores: Scores, // Another struct
    new: bool,
}

fn add_user(name: String, scores: Scores) -> User {
    User {
        _name: name, // field init shorthand(shortens: `name: name,`)
        scores,
        new: true,
    }
}

fn main() {
    // Entire struct must be mutable. Individual fields can't be mutable.
    let mut zim = add_user(String::from("Invader Zim"), Scores(0, 3.6, 9));
    let gir = User {
        _name: String::from("Gir"),
        ..zim // struct update syntax.
    };
    zim.new = false; // Updating a value
    dbg!(gir.scores.2);
    // dbg!(zim.scores.2); // Error!
    dbg!(zim.new);
}
// ANCHOR: all
