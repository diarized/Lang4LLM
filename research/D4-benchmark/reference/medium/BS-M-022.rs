#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn reverse_words(s: String) -> String {
    let raw_words: Vec<String> = s.split(" ".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    let mut words: Vec<String> = vec![];
    for i in 0_i64..(raw_words.len() as i64) {
        if raw_words[(i) as usize].clone() != "".to_string() {
            words.push(raw_words[(i) as usize].clone());
        };
    }
    if words.is_empty() {
        return "".to_string();
    };
    let n: i64 = (words.len() as i64);
    let mut reversed: Vec<String> = vec![];
    for i in 0_i64..n {
        reversed.push(words[(n - 1_i64 - i) as usize].clone());
    }
    let mut result: String = "".to_string();
    for i in 0_i64..(reversed.len() as i64) {
        if i > 0_i64 {
            result = format!("{} ", result);
        };
        result = result + reversed[(i) as usize].clone();
    }
    return result;
}

fn main() {
    println!("{}", format!("reverse_words(\"Hello World\") = {}", reverse_words("Hello World".to_string())));
    println!("{}", format!("reverse_words(\"the quick brown fox\") = {}", reverse_words("the quick brown fox".to_string())));
    println!("{}", format!("reverse_words(\"  hello world  \") = {}", reverse_words("  hello world  ".to_string())));
    println!("{}", format!("reverse_words(\"foo   bar   baz\") = {}", reverse_words("foo   bar   baz".to_string())));
    println!("{}", format!("reverse_words(\"lonely\") = {}", reverse_words("lonely".to_string())));
    println!("{}", format!("reverse_words(\"\") = {}", reverse_words("".to_string())));
    println!("{}", format!("reverse_words(\"     \") = {}", reverse_words("     ".to_string())));
    println!("{}", format!("reverse_words(\"Hello, World!\") = {}", reverse_words("Hello, World!".to_string())));
    println!("{}", format!("reverse_words(\"  one  two  three  \") = {}", reverse_words("  one  two  three  ".to_string())));
    println!("{}", format!("reverse_words(\"a b c d e\") = {}", reverse_words("a b c d e".to_string())));
}

