mod front_of_house;

mod back_of_house {
    use crate::front_of_house::serving::take_payment;
    pub fn fix_incorrect_order() {
        super::front_of_house::serving::serve_order();
        take_payment();
    }
}

pub use back_of_house::fix_incorrect_order;
pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::add_to_waitlist();
    fix_incorrect_order();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
