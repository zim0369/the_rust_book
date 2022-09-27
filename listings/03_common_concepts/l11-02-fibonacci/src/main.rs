//ANCHOR: all
fn fibox(n: u32) -> u32 {
    match n {
        1 => 0,
        2 => 1,
        _ => fibox(n - 1) + fibox(n - 2),
    }
}

fn main() {
    let mut fibo = String::new();
    std::io::stdin().read_line(&mut fibo).unwrap();
    let fibo = fibox(fibo.trim().parse::<u32>().unwrap());
    print!("{fibo}");
}
//ANCHOR_END: all
