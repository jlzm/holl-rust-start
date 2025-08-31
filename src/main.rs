use workspace::front_of_house::hosting;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> i32 {
            99
        }
    }
}

fn main() {
    let lib_val = hosting::add_to_waitlist();
    let self_val = crate::front_of_house::hosting::add_to_waitlist();
    println!("lib val: {}", lib_val);
    println!("self val: {}", self_val);
}
