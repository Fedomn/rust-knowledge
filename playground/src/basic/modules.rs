#[allow(dead_code)]
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

#[allow(dead_code)]
pub fn eat_at_restaurant() {
    // Absolute path
    crate::basic::modules::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();
    // super get parent module path, like filesystem path .. syntax
    super::modules::front_of_house::hosting::add_to_waitlist();
}
