fn main() {
    let number = 3;
    
    if number < 1 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = true;
    let number = if condition {5} else {6};
    // wrong!
    // let number = if condition 5 else 6;
    println!("The value of number is: {}", number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    return_value_from_loops();

    show_while();

    show_for();

    show_rev();
}

fn return_value_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 100000;
        }
    };

    println!("The result is {}", result);
}

fn show_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn show_for() {
    let a = [0, 1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for num in a {
        println!("the value is: {}", num);
    }
}

fn show_rev() {
    for num in (1..100).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!!");
}