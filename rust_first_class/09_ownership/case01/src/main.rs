fn main() {
    let arr = vec![1];
  
    std::thread::spawn(move || {
      println!("{:?}", arr);
    });

    println!("{}", 2);    
  }