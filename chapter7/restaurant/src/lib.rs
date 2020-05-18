mod r#pub; // Import stuff from my pub.rs

// Key takeaway: modules can contain other modules, like a filesystem and dirs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}


// Absolute module paths are preferred as they move calls and code definitions independently
// Items in parent cant use the private inside child modules, but items in child modules can use
// stuff in parent items
