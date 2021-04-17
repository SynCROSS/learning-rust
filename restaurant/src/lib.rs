// * Absolute path: It starts from a crate root by using a crate name or a literal crate.
// * Relative path: It starts from the current module and uses self, super, or an identifier in the current module.

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
