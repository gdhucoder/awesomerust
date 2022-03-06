fn main() {

    // 活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];

    println!("data[0]: {:?}", &data[0]);

    for i in 0..100 {
        data.push(i);
    }
    println!("data[0]: {:?}", &data[0]);
    println!("boxed {:?}", &data1);
}
