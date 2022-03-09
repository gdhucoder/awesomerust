fn main() {
    // test1();
    test2();
}

// fn test1() {
//     let x;
//     {
//         let y = 42;
//         x = &y;
//     }

//     println!("x: {}", x);
// }

fn test2() {
    let y = 42;
    let x = &y;
    println!("x: {}", x);
}