#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_balanced_digits(n: i64) -> bool {
    let digit_chars: Vec<String> = format!("{}", n).chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let d: i64 = digit_chars.len();
    if d <= 1_i64 {
        return true;
    };
    let half: i64 = d / 2_i64;
    let mut left_sum: i64 = 0_i64;
    let mut right_sum: i64 = 0_i64;
    for i in 0_i64..half {
        left_sum = left_sum + { let __s = format!("{}", (digit_chars[i as usize] as f64)); __s.parse::<i64>().unwrap_or(0) };
    }
    for i in d - half..d {
        right_sum = right_sum + { let __s = format!("{}", (digit_chars[i as usize] as f64)); __s.parse::<i64>().unwrap_or(0) };
    }
    return left_sum == right_sum;
}

fn main() {
    println!("{}", format!("is_balanced_digits(1230) = {}", format!("{}", is_balanced_digits(1230_i64))));
    println!("{}", format!("is_balanced_digits(1231) = {}", format!("{}", is_balanced_digits(1231_i64))));
    println!("{}", format!("is_balanced_digits(12321) = {}", format!("{}", is_balanced_digits(12321_i64))));
    println!("{}", format!("is_balanced_digits(7) = {}", format!("{}", is_balanced_digits(7_i64))));
    println!("{}", format!("is_balanced_digits(0) = {}", format!("{}", is_balanced_digits(0_i64))));
    println!("{}", format!("is_balanced_digits(13) = {}", format!("{}", is_balanced_digits(13_i64))));
}

