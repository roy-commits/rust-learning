fn main() {
    let width1 = 50;
    let height1 = 30;
    println!("The area of the rectangle is {} square pixels.",
             area(width1, height1));

    // 使用元组重构
    let rect1 = (50, 50);
    println!("The area of the rectangle is {} square pixels.",
             area_tuple(rect1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
