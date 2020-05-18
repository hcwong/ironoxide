mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // need a public "constructor" because else we cannot instantiate the fruit field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // This is a public enum. usually enums are all public
    // Structs are usually private by default
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// For functions, don't use the whole path but for stuff like structs and enums its idiomatic to
// use the whole thing, unless they have different scopes but the same name, like fmt::Result and
// io::Result
//
// You can also rename imports using as
// pub use is for reexporting in a single line
// Theres also the glob operator import .* in java and nested paths, std::io{self, cmp::Ordering}
