#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn repeat_string(text: String, count: i64) -> String {
    let mut result: String = "".to_string();
    for i in 0_i64..count {
        result = format!("{}{}", format!("{}", result), text);
    }
    return result;
}

fn main() {
    println!("{}", repeat_string("ab".to_string(), 3_i64));
    println!("{}", repeat_string("hello".to_string(), 0_i64));
    println!("{}", repeat_string("hello".to_string(), 1_i64));
    println!("{}", repeat_string("".to_string(), 5_i64));
    println!("{}", repeat_string("x".to_string(), 4_i64));
    println!("{}", repeat_string("ha ".to_string(), 2_i64));
}

