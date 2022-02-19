fn main() {
    let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    if let x = 5 {
        println!("{}", x);
    }

    let  x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Match, y = {:?}", z),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        1|2 => println!("one or two"),
        3 => println!("3"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!(">1 and <5"),
        _ => println!("anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ascii letter"),
        'k'..='z' => println!("late ascii letter"),
        _ => println!("something else"),
    }

    let p = Point {x:0, y:7};
    let Point {x:a, y:b} = p;

    assert_eq!(0, a);

    assert_eq!(7, b);

    match p {
        Point {x, y:0} => println!("On the x axis at {}", x),
        Point {x:0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y),
    }

    //let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::Write(String::from("abc"));
    match msg {
        Message::ChangeColor(r, g, b) => println!(
            "Change the color the red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::Write(text) => println!("Text message: {}", text),
        _ => println!("default"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}