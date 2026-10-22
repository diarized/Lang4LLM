#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn move_zeros_to_end(numbers: Vec<i64>) -> Vec<i64> {
    let mut non_zeros: Vec<i64> = vec![];
    let mut zero_count: i64 = 0_i64;
    for i in 0_i64..numbers.len() {
        if numbers[i as usize] != 0_i64 {
            non_zeros.push(numbers[i as usize]);
        } else {
            zero_count = zero_count + 1_i64;
        };
    }
    for i in 0_i64..zero_count {
        non_zeros.push(0_i64);
    }
    return non_zeros;
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
    let a: Vec<i64> = vec![0_i64, 1_i64, 0_i64, 3_i64, 12_i64];
    println!("{}", format!("move_zeros_to_end([0,1,0,3,12]) = {}", print_int_arr(move_zeros_to_end(a))));
    let b: Vec<i64> = vec![1_i64, 2_i64, 3_i64];
    println!("{}", format!("move_zeros_to_end([1,2,3]) = {}", print_int_arr(move_zeros_to_end(b))));
    let c: Vec<i64> = vec![0_i64, 0_i64, 0_i64];
    println!("{}", format!("move_zeros_to_end([0,0,0]) = {}", print_int_arr(move_zeros_to_end(c))));
    let d: Vec<i64> = vec![5_i64, 6_i64, 0_i64];
    println!("{}", format!("move_zeros_to_end([5,6,0]) = {}", print_int_arr(move_zeros_to_end(d))));
    let e: Vec<i64> = vec![0_i64, -3_i64, 0_i64, 5_i64, -1_i64, 0_i64];
    println!("{}", format!("move_zeros_to_end([0,-3,0,5,-1,0]) = {}", print_int_arr(move_zeros_to_end(e))));
    let f: Vec<i64> = vec![];
    println!("{}", format!("move_zeros_to_end([]) = {}", print_int_arr(move_zeros_to_end(f))));
}

