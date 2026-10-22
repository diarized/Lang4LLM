#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn find_minimum(numbers: Vec<i64>) -> i64 {
    let mut min_val: i64 = numbers[0_i64 as usize];
    for i in 1_i64..numbers.len() {
        if numbers[i as usize] < min_val {
            min_val = numbers[i as usize];
        };
    }
    return min_val;
}

fn main() {
    let a: Vec<i64> = vec![3_i64, 7_i64, 2_i64, 9_i64, 1_i64];
    println!("{}", format!("find_minimum([3,7,2,9,1]) = {}", format!("{}", find_minimum(a))));
    let b: Vec<i64> = vec![-5_i64, -2_i64, -8_i64, -1_i64];
    println!("{}", format!("find_minimum([-5,-2,-8,-1]) = {}", format!("{}", find_minimum(b))));
    let c: Vec<i64> = vec![-3_i64, 5_i64, -1_i64, 2_i64];
    println!("{}", format!("find_minimum([-3,5,-1,2]) = {}", format!("{}", find_minimum(c))));
    let d: Vec<i64> = vec![42_i64];
    println!("{}", format!("find_minimum([42]) = {}", format!("{}", find_minimum(d))));
    let e: Vec<i64> = vec![4_i64, 4_i64, 4_i64, 4_i64];
    println!("{}", format!("find_minimum([4,4,4,4]) = {}", format!("{}", find_minimum(e))));
    let f: Vec<i64> = vec![10_i64, 1_i64, 100_i64, 50_i64];
    println!("{}", format!("find_minimum([10,1,100,50]) = {}", format!("{}", find_minimum(f))));
}

