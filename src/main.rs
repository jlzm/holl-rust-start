use workspace::front_of_house::hosting;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> i32 {
            99
        }
    }
}

struct Person {
    name: String,
    age: u8,
}

use std::fmt;

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hello i'm customize formatter: name-{}, age-{}",
            self.name, self.age
        )
    }
}

fn main() {
    let lib_val = hosting::add_to_waitlist();
    let self_val = crate::front_of_house::hosting::add_to_waitlist();
    println!("lib val: {}", lib_val);
    println!("self val: {}", self_val);

    let p_item = Person {
        name: "xiaoyueyue".to_string(),
        age: 7,
    };

    println!("{}", p_item);
}
