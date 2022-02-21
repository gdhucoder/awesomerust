fn main() {
    println!("Hello, world!");
}

/// never type
fn bar() -> ! {
    // --snip--
    panic!();
}
