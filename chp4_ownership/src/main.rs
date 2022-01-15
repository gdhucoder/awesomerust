fn main() {
    let mut s = String::from("你好");
    s.push_str(" China");
    println!("{}", s);
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    take_ownership(s1);

    // println!("{}", s1); // compile error! s1 has lost its ownership!

    let s1 = String::from("国");
    let (s2, len) = calc_string(s1);
    println!("{} length is: {}", s2, len);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn calc_string(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}