mod front_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table () {}
    }
    mod serving {
        fn take_order () {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //Абсолютный путь
    crate::front_house::hosting::add_to_waitlist();

    // Относительный путь
    front_house::hosting::add_to_waitlist();
}
