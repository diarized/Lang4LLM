#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn rotate_left(numbers: Vec<i64>, k: i64) -> Vec<i64> {
    let n: i64 = numbers.len();
    if n == 0_i64 {
        return numbers;
    };
    let shift: i64 = k % n;
    let mut result: Vec<i64> = vec![];
    for i in shift..n {
        result.push(numbers[i as usize]);
    }
    for i in 0_i64..shift {
        result.push(numbers[i as usize]);
    }
    return result;
}

fn print_int_arr(arr: Vec<i64>) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    };
    let mut parts: Vec<String> = vec![];
    for i in 0_i64..arr.len() {
        parts.push(format!("{}", arr[i as usize]));
    }
    return format!("[{}]", parts.join(", ".to_string().as_str()));
}

fn main() {
    let a: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    println!("{}", format!("rotate_left([1,2,3,4,5], 2) = {}", print_int_arr(rotate_left(a, 2_i64))));
    let b: Vec<i64> = vec![1_i64, 2_i64, 3_i64];
    println!("{}", format!("rotate_left([1,2,3], 3) = {}", print_int_arr(rotate_left(b, 3_i64))));
    let c: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64];
    println!("{}", format!("rotate_left([1,2,3,4], 6) = {}", print_int_arr(rotate_left(c, 6_i64))));
    let d: Vec<i64> = vec![7_i64, 8_i64, 9_i64];
    println!("{}", format!("rotate_left([7,8,9], 0) = {}", print_int_arr(rotate_left(d, 0_i64))));
    let e: Vec<i64> = vec![];
    println!("{}", format!("rotate_left([], 5) = {}", print_int_arr(rotate_left(e, 5_i64))));
    let f: Vec<i64> = vec![10_i64];
    println!("{}", format!("rotate_left([10], 3) = {}", print_int_arr(rotate_left(f, 3_i64))));
}

