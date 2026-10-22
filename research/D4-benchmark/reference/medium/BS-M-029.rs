#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn roman_values() -> Vec<i64> {
    return vec![1000_i64, 900_i64, 500_i64, 400_i64, 100_i64, 90_i64, 50_i64, 40_i64, 10_i64, 9_i64, 5_i64, 4_i64, 1_i64];
}

fn roman_symbols() -> Vec<String> {
    return vec!["M".to_string(), "CM".to_string(), "D".to_string(), "CD".to_string(), "C".to_string(), "XC".to_string(), "L".to_string(), "XL".to_string(), "X".to_string(), "IX".to_string(), "V".to_string(), "IV".to_string(), "I".to_string()];
}

fn to_roman(n: i64) -> String {
    if n < 1_i64 || n > 3999_i64 {
        return "".to_string();
    };
    let values: Vec<i64> = roman_values();
    let symbols: Vec<String> = roman_symbols();
    let mut remaining: i64 = n.clone();
    let mut result: String = "".to_string();
    for i in 0_i64..(values.len() as i64) {
        while remaining >= values[(i) as usize].clone() {
            result = result + symbols[(i) as usize].clone();
            remaining = remaining - values[(i) as usize].clone();
        }
    }
    return result;
}

fn main() {
    println!("{}", format!("to_roman(1) = {}", to_roman(1_i64)));
    println!("{}", format!("to_roman(3) = {}", to_roman(3_i64)));
    println!("{}", format!("to_roman(4) = {}", to_roman(4_i64)));
    println!("{}", format!("to_roman(9) = {}", to_roman(9_i64)));
    println!("{}", format!("to_roman(40) = {}", to_roman(40_i64)));
    println!("{}", format!("to_roman(90) = {}", to_roman(90_i64)));
    println!("{}", format!("to_roman(400) = {}", to_roman(400_i64)));
    println!("{}", format!("to_roman(900) = {}", to_roman(900_i64)));
    println!("{}", format!("to_roman(1000) = {}", to_roman(1000_i64)));
    println!("{}", format!("to_roman(500) = {}", to_roman(500_i64)));
    println!("{}", format!("to_roman(100) = {}", to_roman(100_i64)));
    println!("{}", format!("to_roman(50) = {}", to_roman(50_i64)));
    println!("{}", format!("to_roman(10) = {}", to_roman(10_i64)));
    println!("{}", format!("to_roman(5) = {}", to_roman(5_i64)));
    println!("{}", format!("to_roman(1994) = {}", to_roman(1994_i64)));
    println!("{}", format!("to_roman(3999) = {}", to_roman(3999_i64)));
    println!("{}", format!("to_roman(1949) = {}", to_roman(1949_i64)));
}

