use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 40);
    println!("scores: {:?}", &scores);
    // 遍历hashMap
    for(key, value) in &scores {
        println!("key: {}, value: {}", key, value);
    }

    let blue_score = scores.get("Blue");
    dbg!("the Blue score is: {}", blue_score);

    // 更新HashMap
    scores.insert(String::from("Blue"), 25);
    println!("blue :{:?}", &scores);

    // 只在键值为空时进行插入
    scores.entry(String::from("Red")).or_insert(60);
    scores.entry(String::from("Blue")).or_insert(30);
    println!("scores: {:?}", &scores);

    // 根据旧值更新一个值
    let text = "hello.txt world my wonderful world";
    let mut map = HashMap::new();
    for world in text.split_whitespace() {
        let count = map.entry(world).or_insert(0);
        *count += 1;
    }
    println!("map: {:?}", map);
}
