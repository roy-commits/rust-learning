
// trait 定义是一种将方法签名组合起来的方法
// 目的是定义一个实现某些目的所必需的行为的集合
pub trait Summary{
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Summary具体实现
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    /*fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }*/
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /*fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }*/
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait作为参数
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}