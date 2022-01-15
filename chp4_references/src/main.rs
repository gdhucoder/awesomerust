fn main() {
    let s = String::from("abcd");
    let len = calc_length(&s);
    println!("length is {}", len);
}


fn calc_length(s: &String) -> usize {
    s.len()
}