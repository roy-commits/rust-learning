fn main() {
    /*
        当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源
     */
    let s1 = "hello.txt";
    let s2 = s1.clone();
    println!("s1 = {} \n s2 = {}", s1, s2);

    /*
        像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。
        这意味着没有理由在创建变量 y 后使 x 无效。换句话说，这里没有深浅拷贝的区别，
        所以这里调用 clone 并不会与通常的浅拷贝有什么不同，我们可以不用管它。
     */
    let x = 1;
    let y = x;
    println!("x = {} \n y = {}", x, y);
    /*
        Rust 有一个叫做 Copy trait 的特殊标注，可以用在类似整型这样的存储在栈上的类型上（第 10 章详细讲解 trait）。
        如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
        Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
        如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 标注，将会出现一个编译时错误。
        要学习如何为你的类型添加 Copy 标注以实现该 trait，请阅读附录 C 中的 “可派生的 trait”。

        那么哪些类型实现了 Copy trait 呢？你可以查看给定类型的文档来确认
        不过作为一个通用的规则，任何一组简单标量值的组合都可以实现 Copy
        任何不需要分配内存或某种形式资源的类型都可以实现 Copy 。
        如下是一些 Copy 的类型：
            所有整数类型，比如 u32
            布尔类型，bool，它的值是 true 和 false
            所有浮点数类型，比如 f64
            字符类型，char
            元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。
     */
}