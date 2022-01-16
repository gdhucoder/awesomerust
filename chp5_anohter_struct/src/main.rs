fn main() {
    // let width1 = 30;
    // let height1 = 20;
    // let area = area(width1, height1);
    // println!("{}", area);

    // let dim = (17, 17);
    // println!("{}", area(dim));
    let ret1 = Rectangle {
        width: dbg!(88),
        height: 89
    };

    println!("{}", area(&ret1));

    println!("{:#?}", ret1);

    dbg!(&ret1);

    println!("{}", ret1.area());

    let other = Rectangle {
        width: 100,
        height: 99,
    };

    println!("can hold {}", ret1.can_hold(&other));

    let sq1 = Rectangle::square(100);
    println!("square is {:#?}", sq1);
}

#[derive(Debug)]
struct  Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
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