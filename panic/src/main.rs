use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");

    // panic backtrace
    let v = vec![1, 2, 3];
    println!("v: {:?}", v);
    /*
        使用 RUST_BACKTRACE=1 cargo run 进行异常定位
     */
    // let value = v[99];
    // println!("value: {}", value);

    // Result与可恢复错误
    let file = File::open("hello.txt");
    // 根据File::open返回值进行不同处理逻辑
    let f = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("file: {:?}", f);

    // 精简写法
    let path = String::from("2022-06-13.txt");
    let file = File::open(&path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&path).unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("file: {:?}", file);
}
