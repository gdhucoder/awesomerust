fn main() {
    let data = vec![10, 42, 9, 8];

    let v = 42;

    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
    // Move寓意：赋值或者传参会导致值Move，所有权被转移，
    // 一但所有权转移，之前的变量就不能访问
    // println!("sum of data:{} ", sum(data));
    
    // Copy语义，如果值实现的Copy trait，那么赋值或者传参会使用
    // Copy语义，相应的值会被按位拷贝（浅拷贝），产生新的值
    show(v);
    println!("{}", v);

    // 堆上的数据引用栈上的数据
    // 简单来说就是在堆上数据被回收之前栈上的数据一定会存在的情况下，是可以的
    let x = 1;
    let y = 2;
    let v = vec![&x, &y];
    println!("{:?}", v);
}

fn show(v: u32) {
    println!("{}", v);
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc+x)
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v{
            return Some(pos);
        }
    }

    None
}
