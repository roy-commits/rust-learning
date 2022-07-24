fn main() {
    /*
    提到过当变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存。
    不过s1和s2两个数据指针指向了同一位置。这就有了一个问题：当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。
    这是一个叫做 二次释放（double free）的错误，也是之前提到过的内存安全性 bug 之一。
    两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
    为了确保内存安全，这种场景下 Rust 的处理有另一个细节值得注意。
    在 let s2 = s1 之后，Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西
    Rust 禁止使用无效的引用， 只有 s2 是有效的，当其离开作用域，它就释放自己的内存，以上。
    ***** 另外，这里还隐含了一个设计选择：Rust 永远也不会自动创建数据的 “深拷贝”。
          因此，任何 自动 的复制可以被认为对运行时性能影响较小（尽量减少内存占用） *****
     */
    // let s1 = String::from("hello.txt");
    // let s2 = s1;
    // println!("s1 = {}", s1);

    // 所有权与函数示例
    let str = String::from("hello.txt");    // str进入作用域
    take_ownership(str);    // str的值转移到函数中
    // str值不再有效
    // println!("str = {}", str);

    let x= 5;   // x进入作用域
    makes_copy(x);  // 作用域转移到函数中
    println!("original:{}", x);   // 基本数据类型使用copy机制，方便后续继续使用
}   // main函数执行完毕，x作用域结束，执行`drop`，释放占用内存； str作用域在执行过程中已经在对应函数中释放，此处不再处理

fn take_ownership(some_string: String){ // str进入作用域
    println!("{}", some_string);
}   // 函数执行完毕后，str作用域结束，Rust会调用`drop`，str占用内存会被即刻释放（内存释放是由守护线程执行还是由一组特定的线程执行？）

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}