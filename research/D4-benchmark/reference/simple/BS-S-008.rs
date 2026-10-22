#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn remove_duplicates(numbers: Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    for i in 0_i64..numbers.len() {
        let mut found: bool = false;
        for j in 0_i64..result.len() {
            if result[j as usize] == numbers[i as usize] {
                found = true;
            };
        }
        if found == false {
            result.push(numbers[i as usize]);
        };
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
    let a: Vec<i64> = vec![1_i64, 2_i64, 2_i64, 3_i64, 1_i64, 4_i64];
    println!("{}", format!("remove_duplicates([1,2,2,3,1,4]) = {}", print_int_arr(remove_duplicates(a))));
    let b: Vec<i64> = vec![5_i64, 3_i64, 8_i64, 1_i64];
    println!("{}", format!("remove_duplicates([5,3,8,1]) = {}", print_int_arr(remove_duplicates(b))));
    let c: Vec<i64> = vec![7_i64, 7_i64, 7_i64, 7_i64];
    println!("{}", format!("remove_duplicates([7,7,7,7]) = {}", print_int_arr(remove_duplicates(c))));
    let d: Vec<i64> = vec![];
    println!("{}", format!("remove_duplicates([]) = {}", print_int_arr(remove_duplicates(d))));
    let e: Vec<i64> = vec![-1_i64, -2_i64, -1_i64, 0_i64, -2_i64];
    println!("{}", format!("remove_duplicates([-1,-2,-1,0,-2]) = {}", print_int_arr(remove_duplicates(e))));
}

