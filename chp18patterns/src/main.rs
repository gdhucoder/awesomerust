fn main() {
    let favoriate_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();

    if let Some(color) = favoriate_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color!");
        } else {
            println!("Using orange as the background color!");
        }
    } else {
        println!("Using blue as the background color!");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (idx, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, idx);
    }

    let (x, y) = (1, 2, 3);
}
