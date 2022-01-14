fn main() {
    println!("Hello, world!");
    // another_function();
    another_function(100);

    let y = {
        let x = 1001;
        x + 100
    };

    println!("the value of y is {}", y);
    
    let mut x = five();
    println!("the value of x is: {}", x);

    x = plus_one(1);
    println!("the value of x is: {}", x);
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x :i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}