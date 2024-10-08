use front_of_house::hosting::add_to_waitlist;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn main() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    // front_of_house::hosting::add_to_waitlist();
    add_to_waitlist();
    // println!("Hello, world!");
}
