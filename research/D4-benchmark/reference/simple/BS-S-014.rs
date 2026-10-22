#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn count_greater_than(numbers: Vec<i64>, threshold: i64) -> i64 {
    let mut count: i64 = 0_i64;
    for i in 0_i64..numbers.len() {
        if numbers[i as usize] > threshold {
            count = count + 1_i64;
        };
    }
    return count;
}

fn main() {
    let a: Vec<i64> = vec![1_i64, 5_i64, 3_i64, 8_i64, 2_i64, 7_i64];
    println!("{}", format!("count_greater_than([1,5,3,8,2,7], 4) = {}", format!("{}", count_greater_than(a, 4_i64))));
    let b: Vec<i64> = vec![1_i64, 2_i64, 3_i64];
    println!("{}", format!("count_greater_than([1,2,3], 5) = {}", format!("{}", count_greater_than(b, 5_i64))));
    let c: Vec<i64> = vec![3_i64, 5_i64, 7_i64];
    println!("{}", format!("count_greater_than([3,5,7], 5) = {}", format!("{}", count_greater_than(c, 5_i64))));
    let d: Vec<i64> = vec![];
    println!("{}", format!("count_greater_than([], 10) = {}", format!("{}", count_greater_than(d, 10_i64))));
    let e: Vec<i64> = vec![-3_i64, -1_i64, 0_i64, 2_i64];
    println!("{}", format!("count_greater_than([-3,-1,0,2], -2) = {}", format!("{}", count_greater_than(e, -2_i64))));
    let f: Vec<i64> = vec![10_i64, 20_i64, 30_i64];
    println!("{}", format!("count_greater_than([10,20,30], 5) = {}", format!("{}", count_greater_than(f, 5_i64))));
}

