fn main() {
    // let width1 = 30;
    // let height1 = 20;
    // let area = area(width1, height1);
    // println!("{}", area);

    // let dim = (17, 17);
    // println!("{}", area(dim));
    let ret1 = Rectangle {
        width: 88,
        height: 89
    };

    println!("{}", area(&ret1));
}

struct  Rectangle {
    width: u32,
    height: u32,
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}