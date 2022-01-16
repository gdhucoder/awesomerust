fn main() {
    // let ref_to_nothing = dangle();
    let ref_no_dangle = no_dangel();
    println!("{}", ref_no_dangle);
}

// fn danlge() -> &String {
//     let s = String::from("Hello");
//     &s;
// }

fn no_dangel() -> String {
    let s = String::from("Hello");
    s
}