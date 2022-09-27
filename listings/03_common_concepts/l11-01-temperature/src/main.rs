//ANCHOR: all
fn main() {
    let mut temp = String::new();
    loop {
        println!("Fahrenheit(f) or Celsius(c)? ");
        let mut unit = String::new();
        std::io::stdin().read_line(&mut unit).unwrap();
        match unit.as_str().trim() {
            "f" | "F" => {
                println!("Enter temp in Fahrenheit:");
                std::io::stdin().read_line(&mut temp).unwrap();
                let temp = temp.trim().parse::<f32>().unwrap();
                let temp: f32 = (temp - 32.0) * (5.0 / 9.0);
                print!("Temp in Celsius: {temp}");
                break;
            }
            "c" | "C" => {
                println!("Enter temp in Celsius:");
                std::io::stdin().read_line(&mut temp).unwrap();
                let temp = temp.trim().parse::<f32>().unwrap();
                let temp: f32 = temp * (9.0 / 5.0) + 32.0;
                print!("Temp in Fahrenheit: {temp}");
                break;
            }
            _ => {
                continue;
            }
        }
    }
}
//ANCHOR_END: all
