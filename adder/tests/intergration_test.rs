use adder::*;
mod common;

#[test]
fn it_adds_two(){
    common::init();
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn exploration() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn larger_can_hold_smaller(){
    let larger = Rectangle { width: 8, height: 8 };
    let smaller = Rectangle { width: 7, height: 5 };
    assert!(larger.can_hold(&smaller));
}