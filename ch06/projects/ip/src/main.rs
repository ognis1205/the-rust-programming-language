#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V5(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V5(String::from("::1"));
    println!("print enum: {:?}", home);
    println!("print enum: {:?}", loopback);
}
