use std::{env, process};
use minigrep::Config;


fn main() {
    // 获取命令行输入内容
    let args :Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // 进程退出
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        // 进程退出
        process::exit(1);
    }
}

