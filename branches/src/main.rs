fn main() {

    let y = 3;

    if y < 5{
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // let语句中使用if
    let condition = true;
    let number = if condition { 5 }else { "six" };
    println!("The value of number is: {}", number);
}
