#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // ANCHOR: enums
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    // ANCHOR_END: enums

    // ANCHOR: funky
    enum Value {
        Nothing,
        Something { field: i32 },
    }
    // ANCHOR_END: funky
}
