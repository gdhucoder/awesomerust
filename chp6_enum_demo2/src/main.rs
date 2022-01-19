fn main() {
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    println!("{:#?}", home);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("{:#?}", loopback);

    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:#?}", home);

    let m = Message::Write(String::from("hello"));
    m.call();
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("calling...");
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}