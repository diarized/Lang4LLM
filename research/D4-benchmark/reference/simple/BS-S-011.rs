#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn list_contains(numbers: Vec<i64>, target: i64) -> bool {
    for i in 0_i64..numbers.len() {
        if numbers[i as usize] == target {
            return true;
        };
    }
    return false;
}

fn main() {
    let a: Vec<i64> = vec![10_i64, 20_i64, 30_i64, 40_i64];
    println!("{}", format!("list_contains([10,20,30,40], 30) = {}", format!("{}", list_contains(a, 30_i64))));
    let b: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64];
    println!("{}", format!("list_contains([1,2,3,4], 9) = {}", format!("{}", list_contains(b, 9_i64))));
    let c: Vec<i64> = vec![];
    println!("{}", format!("list_contains([], 5) = {}", format!("{}", list_contains(c, 5_i64))));
    let d: Vec<i64> = vec![7_i64, 1_i64, 2_i64, 3_i64];
    println!("{}", format!("list_contains([7,1,2,3], 7) = {}", format!("{}", list_contains(d, 7_i64))));
    let e: Vec<i64> = vec![-1_i64, 0_i64, 1_i64];
    println!("{}", format!("list_contains([-1,0,1], 0) = {}", format!("{}", list_contains(e, 0_i64))));
    let f: Vec<i64> = vec![5_i64];
    println!("{}", format!("list_contains([5], 5) = {}", format!("{}", list_contains(f, 5_i64))));
}

