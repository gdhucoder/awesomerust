fn main() {
    let integer = Point{x:1 ,y: 10};
    let float = Point{ x:1.0, y:10.0 };
    // let wont_work = Point {x:10, y:1.0};
    let p = Point{x:1, y:5};
    println!("p.x = {}", p.x());
    println!("p. = {}", p.y());

}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}