use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("Faild to open hello.txt");
}
