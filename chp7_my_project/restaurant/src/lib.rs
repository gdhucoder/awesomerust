#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
// }
mod front_of_house;
use front_of_house::hosting as HOSTING;

fn serve_order () {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // call parent
        super::serve_order();
    }
    fn cook_order(){}

    pub struct Breakfast {
        pub totast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                totast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant () {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.totast == String::from(("Wheat"));
    println!("I'd like {} totast please", meal.totast);
    HOSTING::add_to_waitlist();
}