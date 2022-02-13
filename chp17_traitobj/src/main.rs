use gui::{Button, SelectBox, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox{
                width:75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("Maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // Rust won’t compile our code if the values don’t implement the traits that the trait objects need.
            Box::new(String::from("hi")),
        ],
    };

    screen.run();
}
