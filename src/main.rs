#[derive(Debug)]
struct Name {
    x: u32,
    y: u32,
}

impl Name {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    fn new(x: u32, y: u32) -> Name {
        Name { x, y }
    }

    fn area(&self) -> u32 {
        println!("hi methods return: {}", self.x);
        self.x
    }
}

fn main() {
    let my_name = Name { x: 1, y: 2 };
    let my_age = my_name.area();
    println!("hi: {}", my_age);
}
