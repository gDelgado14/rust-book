fn main() {
    println!("Hello, world!");

    let result = another_function(12, 4);

    println!("The value of result is: {}", result)
}

fn another_function(x: i32, y: i32) -> i32 {
    x + y
}
