//ANCHOR: all
fn main() {
    let a = [1, 2, 3, 4, 5]; // Type: &[i32]

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    // assert_eq!(slice, [2, 3]); // Both work
}
//ANCHOR: all 
