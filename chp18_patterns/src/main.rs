fn main() {
    // 
    // Ignoring an Entire Value with _
    foo(4,5);

    let  _x = 5;
    let  y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);


    let origin = Point{x:0, y:0, z:0};

    match origin {
        Point{x, ..} => println!("x is {}", x),
    }

    let num = Some(5);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}


fn foo(_:i32, y: i32) {
    println!("This code only uses sthe y parameter: {}", y);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}