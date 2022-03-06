fn main() {
    let data = vec![1, 2, 3, 4];
    
    // println!("sum of data {}", sum(data));

    let data1 = &data;

    println!( "addr of value: 
                data {:p}
                data1 ({:p}), 
                addr of data {:p}, 
                data1: {:p}",
     &data, data1, &&data, &data1 );

    println!("sum of data: {}", sum(data1));

    println!( "addr of items: [{:p}, {:p}, {:p}, {:p}]", &data[0], &data[1], &data[2], &data[3] );

    // let r = local_ref();
    // println!("r: {:p}", r);

    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("{:?}", data);

}

fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc+x)
}

// fn local_ref<'a>() -> &'a i32 {
//     let a = 42;
//     // &a
//     //
//     //^^ returns a reference to data owned by the current function
    
// }

fn push_local_ref(data: &mut Vec<&u32>){
    let v =  42;
    data.push(&v);
}