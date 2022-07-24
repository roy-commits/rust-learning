extern crate core;

use std::error::Error;
use std::{env, fs};


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // 校验用户输入参数
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // 获取环境变量 CASE_INSENSITIVE, 只要CASE_INSENSITIVE存在即为true(无论设为何值  )
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // 统一转换为小写
    let lower_case = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&lower_case) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct test.";
        assert_eq!(vec!["safe, fast, productive."],
                   search(query, contents));
    }

    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }
}