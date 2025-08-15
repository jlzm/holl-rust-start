#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: x @ 10, y } = point {
        println!("x is {} and y is {}, p.x is {}", x, y, p.x);
        println!("p is {:?}", p);
    } else {
        println!("x was not 10 :(");
    }

    // 新语法测试
    match 1 {
        num @ (1 | 2) => {
            println!("hi ~:{}", num);
        }
        _ => {}
    }
}
