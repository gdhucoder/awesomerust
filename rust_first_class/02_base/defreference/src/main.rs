fn main() {
    let mut mylist = show;
    mylist();
    mylist = vec![1,2,3];
    println!("{:?}", mylist);
    // 4 |     mylist = vec![1,2,3];
    //   |              ^^^^^^^^^^^ expected fn item, found struct `Vec`
    // 强类型语言Rust，函数指针解引用，指向新的变量，必须也是函数指针类型
}


fn show() {
    println!("this is show");
}