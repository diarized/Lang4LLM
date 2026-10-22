#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn has_substring(text: String, substring: String) -> bool {
    if substring.len() as i64 == 0_i64 {
        return true;
    };
    return text.contains(&substring);
}

fn main() {
    println!("{}", format!("{}", has_substring("hello world".to_string(), "world".to_string())));
    println!("{}", format!("{}", has_substring("hello world".to_string(), "xyz".to_string())));
    println!("{}", format!("{}", has_substring("Hello World".to_string(), "hello".to_string())));
    println!("{}", format!("{}", has_substring("anything".to_string(), "".to_string())));
    println!("{}", format!("{}", has_substring("".to_string(), "abc".to_string())));
    println!("{}", format!("{}", has_substring("".to_string(), "".to_string())));
    println!("{}", format!("{}", has_substring("abc".to_string(), "abc".to_string())));
    println!("{}", format!("{}", has_substring("hello world".to_string(), "hello".to_string())));
    println!("{}", format!("{}", has_substring("hello world".to_string(), "abcdefghijk".to_string())));
}

