#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn find_missing_number(numbers: Vec<i64>) -> i64 {
    let n: i64 = numbers.len() as i64 + 1_i64;
    let expected_sum: i64 = n * (n + 1_i64) / 2_i64;
    let mut actual_sum: i64 = 0_i64;
    for i in 0_i64..numbers.len() as i64 {
        actual_sum = actual_sum + numbers[(i) as usize].clone();
    }
    return expected_sum - actual_sum;
}

fn main() {
    let a: Vec<i64> = vec![1_i64, 2_i64, 4_i64, 5_i64];
    println!("{}", format!("find_missing_number([1,2,4,5]) = {}", format!("{}", find_missing_number(a))));
    let b: Vec<i64> = vec![2_i64, 3_i64, 4_i64];
    println!("{}", format!("find_missing_number([2,3,4]) = {}", format!("{}", find_missing_number(b))));
    let c: Vec<i64> = vec![1_i64, 2_i64, 3_i64];
    println!("{}", format!("find_missing_number([1,2,3]) = {}", format!("{}", find_missing_number(c))));
    let d: Vec<i64> = vec![2_i64];
    println!("{}", format!("find_missing_number([2]) = {}", format!("{}", find_missing_number(d))));
    let e: Vec<i64> = vec![5_i64, 3_i64, 1_i64, 2_i64];
    println!("{}", format!("find_missing_number([5,3,1,2]) = {}", format!("{}", find_missing_number(e))));
    let f: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64, 11_i64];
    println!("{}", format!("find_missing_number([1,2,3,5,6,7,8,9,10,11]) = {}", format!("{}", find_missing_number(f))));
}

