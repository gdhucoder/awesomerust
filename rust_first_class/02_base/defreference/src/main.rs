fn main() {
    let mut mylist = show;
    mylist();
    // mylist = vec![1,2,3];
    // println!("{:?}", mylist);
    // 4 |     mylist = vec![1,2,3];
    //   |              ^^^^^^^^^^^ expected fn item, found struct `Vec`
    // 强类型语言Rust，函数指针解引用，指向新的变量，必须也是函数指针类型

    let shape = Shape::new(EShape::Triangle(3.0, 4.0, 5.0));
    println!("shape: {:#?}", shape);
}


fn show() {
    println!("this is show");
}

struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

#[derive(Debug)]
enum EShape {
    Rectangle(f64, f64),
    Circle(f64),
    Triangle(f64, f64, f64),
}


#[derive(Debug)]
struct Shape {
    shape: EShape,
}


impl Shape {
    fn new(shape: EShape) -> Shape {
        Shape {shape}
    }
}

trait Calculator {
    fn permimeter(&self) -> f64;
}

impl Calculator for Shape {
    fn permimeter(&self) -> f64 {
        match self.shape {
            EShape::Rectangle(a,b) => (a+b)*2.0,
            EShape::Circle(r) => 2.0 * 3.14 * r,
            EShape::Triangle(a,b,c)=> a + b + c,
        }
    }
}
