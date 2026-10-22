#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn capitalize_first(text: String) -> String {
    if text.len() as i64 == 0_i64 {
        return "".to_string();
    };
    let ch: Vec<String> = text.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let first_char: String = ch[(0_i64) as usize].clone().to_uppercase();
    let mut rest: String = "".to_string();
    for i in 1_i64..ch.len() as i64 {
        rest = format!("{}{}", rest, ch[(i) as usize].clone().to_lowercase());
    }
    return format!("{}{}", first_char, format!("{}", rest));
}

fn main() {
    println!("{}", capitalize_first("hello".to_string()));
    println!("{}", capitalize_first("WORLD".to_string()));
    println!("{}", capitalize_first("hELLO wORLD".to_string()));
    println!("{}", capitalize_first("".to_string()));
    println!("{}", capitalize_first("a".to_string()));
    println!("{}", capitalize_first("123abc".to_string()));
    println!("{}", capitalize_first("!!!".to_string()));
}

