#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn capitalize_word(word: String) -> String {
    if word.len() as i64 == 0_i64 {
        return "".to_string();
    };
    let ch: Vec<String> = word.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let first_char: String = ch[(0_i64) as usize].clone().to_uppercase();
    let mut rest: String = "".to_string();
    for i in 1_i64..word.len() as i64 {
        rest = format!("{}{}", rest, ch[(i) as usize].clone().to_lowercase());
    }
    return format!("{}{}", first_char, format!("{}", rest));
}

fn to_title_case(text: String) -> String {
    if text.len() as i64 == 0_i64 {
        return "".to_string();
    };
    let words: Vec<String> = text.split(" ".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    let mut result: Vec<String> = vec![];
    for i in 0_i64..words.len() as i64 {
        result.push(capitalize_word(words[(i) as usize].clone()));
    }
    return result.join(" ".to_string().as_str());
}

fn main() {
    println!("{}", to_title_case("hello world".to_string()));
    println!("{}", to_title_case("Hello World".to_string()));
    println!("{}", to_title_case("HELLO WORLD".to_string()));
    println!("{}", to_title_case("the QUICK brown FOX".to_string()));
    println!("{}", to_title_case("python".to_string()));
    println!("{}", to_title_case("".to_string()));
    println!("{}", to_title_case("version 2 release".to_string()));
    println!("{}", to_title_case("1st place winner".to_string()));
}

