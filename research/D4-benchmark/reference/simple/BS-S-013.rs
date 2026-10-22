#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn linear_search(numbers: Vec<i64>, target: i64) -> i64 {
    for i in 0_i64..numbers.len() {
        if numbers[i as usize] == target {
            return i;
        };
    }
    return -1_i64;
}

fn main() {
    let a: Vec<i64> = vec![4_i64, 7_i64, 2_i64, 9_i64, 1_i64];
    println!("{}", format!("linear_search([4,7,2,9,1], 9) = {}", format!("{}", linear_search(a, 9_i64))));
    let b: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    println!("{}", format!("linear_search([1,2,3,4,5], 6) = {}", format!("{}", linear_search(b, 6_i64))));
    let c: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64, 9_i64];
    println!("{}", format!("linear_search([3,1,4,1,5,9], 1) = {}", format!("{}", linear_search(c, 1_i64))));
    let d: Vec<i64> = vec![8_i64, 2_i64, 5_i64, 3_i64];
    println!("{}", format!("linear_search([8,2,5,3], 8) = {}", format!("{}", linear_search(d, 8_i64))));
    let e: Vec<i64> = vec![];
    println!("{}", format!("linear_search([], 7) = {}", format!("{}", linear_search(e, 7_i64))));
    let f: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    println!("{}", format!("linear_search([1,2,3,4,5], 5) = {}", format!("{}", linear_search(f, 5_i64))));
}

