Module

1
```
// in lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        fn complain() {} 
    }
}
```

2
```
// in lib.rs
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}

        // Maybe you don't want the guest hearing the your complaining about them
        // So just make it private
        fn complain() {} 
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```


3
```
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
```

4
```
// in src/lib.rs

mod front_of_house;
mod back_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}

// in src/back_of_house.rs

use crate::front_of_house;
pub fn fix_incorrect_order() {
    cook_order();
    front_of_house::serving::serve_order();
}

pub fn cook_order() {}

// in src/front_of_house/mod.rs

pub mod hosting;
pub mod serving;

// in src/front_of_house/hosting.rs

pub fn add_to_waitlist() {}

pub fn seat_at_table() -> String {
    String::from("sit down please")
}

// in src/front_of_house/serving.rs

pub fn take_order() {}

pub fn serve_order() {}

pub fn take_payment() {}

// Maybe you don't want the guest hearing the your complaining about them
// So just make it private
fn complain() {} 
```

accessing code in library crate from binary crate

5
```
mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}

```

