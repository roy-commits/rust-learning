fn main() {
    // 表达式赋值
    let y = {
        let x = 5;
        x + 1
    };
    println!("The value of y is: {}", y);

   // 带有返回值的函数
   let result = five();
    println!("The value of five function: {}", result);

    // 带有入参并有返回值的函数
   let result = plus_one(5);
    println!("The value of plus_one is: {}", result);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}
