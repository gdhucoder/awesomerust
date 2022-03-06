fn main() {
    let mut arr = vec![1,4];
    // 一个值可以有唯一一个活跃的可变引用。
    // 可变引用（写）和只读引用（读）是互斥的关系，就像并发下数据的读写互斥那样。
    let last = arr.last();
    println!("last: {:?}", last);
    arr.push(10);
}
