fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home is :{:#?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("loopback is :{:#?}", loopback);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:#?} routing...", ip_kind);
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}