use std::sync::Arc;
fn main() {
    let str = Arc::new(String::from("hello"));

    {
        let str = str.clone();
        std::thread::spawn(move||{
            println!("{:?}", str);
        });
    }


    println!("main: {}", str);

}
