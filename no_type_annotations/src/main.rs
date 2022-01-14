fn main() {
    let guess:u8 = "42".parse().expect("Not a number");

    let y: f64 = 3.0;
    println!("y is {}", y);

    // floored
    let floored = 5/3; // 向下取整
    println!("Floored {}", floored);

    // remainder
    let remainder = 34 % 10;
    println!("remainder is {} ", remainder);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is {:?}", tup);

    let mut arr = [3; 5];
    println!("array value is {:?}", arr);
    arr[0] = 10;
    println!("array value is {:?}", arr);

    println!("invalid array access, arr[10] {}", arr[10]);
}
