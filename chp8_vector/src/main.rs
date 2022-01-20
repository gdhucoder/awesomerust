fn main() {
    println!("Hello, world!");
    //let v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3];
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("the third element is {}", v[3]);

    // match v.get(100) {
    //     Some(third) => println!("the third element is {}", third),
    //     None => println!("there is not third element!"),
    // }

    // let none = v[1000];
    // println!("{}", none);

    let mut v = vec![1,2,3,4,5,6];
    let first = &v[0];
    //v.push(10);
    println!("the frist element is {}", first);
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

}
