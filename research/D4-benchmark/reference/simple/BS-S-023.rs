#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn trim_whitespace(text: String) -> String {
    return text.trim().to_string();
}

fn main() {
    println!("{}", trim_whitespace("  hello  ".to_string()));
    println!("{}", trim_whitespace("   hello".to_string()));
    println!("{}", trim_whitespace("hello   ".to_string()));
    println!("{}", trim_whitespace("  hello world  ".to_string()));
    println!("{}", trim_whitespace("hello".to_string()));
    println!("{}", trim_whitespace("   ".to_string()));
    println!("{}", trim_whitespace("".to_string()));
    println!("{}", trim_whitespace("  tab content  ".to_string()));
}

