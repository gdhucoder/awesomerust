fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("{}", hello);
    let world = &s[6..s.len()];
    println!("{}", world);

    let s1 = first_word(&s);
    let word = first_word(&s[..]);

    let my_string_literal = "hello world";

    let word = first_word(my_string_literal);
    let word = first_word(&my_string_literal);
    println!("{}", word);

    array_lice();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn array_lice() {
    let a = [0, 1, 2, 3, 4];
    let slice = &a[0..2];
    assert_eq!(slice, &[0,1,]);
}