fn main() {
    let s = String::from("abcd");
    let len = calc_length(&s);
    println!("length is {}", len);
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}{}", r1, r2);
}


fn calc_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("hello!");
}