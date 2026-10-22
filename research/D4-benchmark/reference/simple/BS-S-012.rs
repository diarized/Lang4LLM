#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn get_last(numbers: Vec<i64>) -> i64 {
    return numbers[numbers.len() as i64 - 1_i64 as usize];
}

fn print_last(numbers: Vec<i64>) -> String {
    if numbers.is_empty() {
        return "none".to_string();
    };
    return format!("{}", get_last(numbers));
}

fn main() {
    let a: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    println!("{}", format!("get_last([1,2,3,4,5]) = {}", print_last(a)));
    let b: Vec<i64> = vec![42_i64];
    println!("{}", format!("get_last([42]) = {}", print_last(b)));
    let c: Vec<i64> = vec![];
    println!("{}", format!("get_last([]) = {}", print_last(c)));
    let d: Vec<i64> = vec![-10_i64, -3_i64, -7_i64];
    println!("{}", format!("get_last([-10,-3,-7]) = {}", print_last(d)));
    let e: Vec<i64> = vec![5_i64, 3_i64, 1_i64, 0_i64];
    println!("{}", format!("get_last([5,3,1,0]) = {}", print_last(e)));
}

