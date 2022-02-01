fn main() {
    let number_list = vec![52, 32, 33, 100, 1];
    let result = largest_i32(&number_list);
    
    println!("The largest number is {}", result);

    let number_list = vec![52, 10000, 33, 100, 1];
    let result = largest_i32(&number_list);
    
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'z', 'Z', 'c'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let char_list = vec!['a', 'z', 'Z', 'c'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/// 实现PartialOrd和Copy
fn largest<T: PartialOrd + Copy>(list: &[T]) ->T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    // 使用引用&，得到值
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}