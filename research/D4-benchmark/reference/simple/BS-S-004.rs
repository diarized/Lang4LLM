#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn sum_list(numbers: Vec<i64>) -> i64 {
    let mut total: i64 = 0_i64;
    for i in 0_i64..numbers.len() {
        total = total + numbers[i as usize];
    }
    return total;
}

fn main() {
    let a: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    println!("{}", format!("sum_list([1,2,3,4,5]) = {}", format!("{}", sum_list(a))));
    let b: Vec<i64> = vec![-1_i64, -2_i64, -3_i64];
    println!("{}", format!("sum_list([-1,-2,-3]) = {}", format!("{}", sum_list(b))));
    let c: Vec<i64> = vec![10_i64, -5_i64, 3_i64, -2_i64];
    println!("{}", format!("sum_list([10,-5,3,-2]) = {}", format!("{}", sum_list(c))));
    let d: Vec<i64> = vec![];
    println!("{}", format!("sum_list([]) = {}", format!("{}", sum_list(d))));
    let e: Vec<i64> = vec![42_i64];
    println!("{}", format!("sum_list([42]) = {}", format!("{}", sum_list(e))));
    let f: Vec<i64> = vec![0_i64, 0_i64, 5_i64, 0_i64];
    println!("{}", format!("sum_list([0,0,5,0]) = {}", format!("{}", sum_list(f))));
}

