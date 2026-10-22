#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn reverse_string(s: String) -> String {
    let mut char_arr: Vec<String> = s.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    char_arr.reverse();
    return char_arr.join("".to_string().as_str());
}

fn main() {
    println!("{}", format!("reverse_string("hello") = {}", reverse_string("hello".to_string())));
    println!("{}", format!("reverse_string("hello world") = {}", reverse_string("hello world".to_string())));
    println!("{}", format!("reverse_string("racecar") = {}", reverse_string("racecar".to_string())));
    println!("{}", format!("reverse_string("") = {}", reverse_string("".to_string())));
    println!("{}", format!("reverse_string("a") = {}", reverse_string("a".to_string())));
    println!("{}", format!("reverse_string("abc!") = {}", reverse_string("abc!".to_string())));
}

