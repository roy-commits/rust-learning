use std::ops::Index;
use std::os::raw::c_char;

fn main() {
    // double_loop();
    // let result = loop_return();
    // println!("result = {}", result);
    // while_loop();
    // array_loop();
    // for_rev();
    print_word();
}

fn double_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn loop_return() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            return counter * 2;
        }
    };
}

fn while_loop(){
    let mut number = 3;
    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    }
    println!("LIFT OFF !!!!")
}

fn array_loop(){
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < array.len() {
        println!("index = {}, array[index] = {}", index, array[index]);
        index += 1;
    }
}

fn for_rev(){
    for number in (1..4).rev()  {
        println!("{}!", number);
    }
    println!("LIFT oFF !!!!")
}

fn print_word(){
    let word = "The Twelve Days of Christmas";
    let mut distinct_char = String::new();
    let mut repeated_char = String::new();
    for char in word.chars() {
        if distinct_char.contains(char) {
            repeated_char.push(char);
        }else {
            distinct_char.push(char);
        }
    }
    println!("distinct_char: {}", distinct_char);
    println!("repeated_char: {}", repeated_char);
}