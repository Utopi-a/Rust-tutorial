#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");

    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    println!("four is {:#?}", four);
    println!("six is {:#?}", six);
}

fn route(ip_type: IpAddr) {
    println!("Routing IP address of type: {:#?}", ip_type);
}
