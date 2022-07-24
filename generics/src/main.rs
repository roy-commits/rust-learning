use generics::{NewArticle, Summary, Tweet};

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // 不使用泛型处理重复问题
    let number_list = vec![34, 50, 33, 100, 66];
    /* let mut largest = number_list[0];
     for number in number_list {
         if number > largest {
             largest = number;
         }
     }*/
    // 当有多个列表时会存在多个整数集合及相关判断逻辑
    // 为消除重复，可以通过抽象的方式，对获取到的任意整形列表作为参数
    // 并对其进行处理
    /*let result = find_largest_number(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![100, 444, 33, 50, 559, 772];
    let result = find_largest_number(&number_list);
    println!("The largest number is {}", result);*/

    // 目前针对同类型的列表可以进行重复性代码合并，如果需要对不同数据类型的列表进行同样的操作，该如何实现？
    // 如果我们有两个函数，一个寻找 i32 值的 slice 中的最大项，而另一个寻找 char 值的 slice 中的最大项，该怎么办？该如何消除重复呢？
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'h', 'r', 'w'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // largest_i32与largest_char之间的区别仅在于数据类型，函数处理逻辑一摸一样
    // 可以定义一个函数，引进泛型参数进一步消除冗余
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 泛型使用案例
    let p1 = Point {
        x: 5,
        y: 8.6,
    };

    let p2 = Point {
        x: "Hello",
        y: 'D',
    };
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as your probably already know, people"),
        reply: true,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // summarize默认实现
    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());
}

// 使用泛型作为入参/出参
fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn find_largest_number(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 泛型数据类型
// 在函数定义中使用泛型，为函数提供更强的使用范围及适配性
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}