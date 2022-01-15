fn main() {
    // let mut x = 5;
    // println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // println!("three hours in seconds {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        // 12
        println!("The value of x in the inner scope is {}", x);
    }
    // 16
    println!("The value of x is {}", x);

    // let spaces = "   ";
    // let spaces = spaces.len();
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("The len is {}", spaces);
}
