#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}
// 元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@emal.cn"),
        sign_in_account: 001,
    };
    user.email = String::from("test@example.com");
    println!("user: {:?}", user);

    // 构造指定字段
    let email = String::from("test001@example.com");
    let username = String::from("test001");
    let user1 = build_user(email, username);
    println!("user1: {:?}", user1);

    // 不使用更新语法从user1创建新的结构体
    let user2 = User {
        email: String::from("test002@example.com"),
        username: user1.username,
        active: user1.active,
        sign_in_account: user1.sign_in_account,
    };
    println!("user2: {:?}", user2);

    // 使用更新语法
    let user3 = User {
        email: String::from("test003@example.com"),
        username: String::from("test003"),
        ..user2
    };

    println!("user3: {:?}", user3);

    // 元组结构体
    let black = Color(0, 0, 0);
    let point = Point(32, 33, 88);
    println!("color: {:?}, point: {:?}", black, point);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_account: 001,
    }
}
