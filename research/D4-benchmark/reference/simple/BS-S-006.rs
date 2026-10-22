#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_sorted(numbers: Vec<i64>) -> bool {
    let n: i64 = numbers.len() as i64;
    if n <= 1_i64 {
        return true;
    };
    for i in 0_i64..n - 1_i64 {
        if numbers[i as usize] > numbers[i + 1_i64 as usize] {
            return false;
        };
    }
    return true;
}

fn main() {
    let a: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    println!("{}", format!("is_sorted([1,2,3,4,5]) = {}", format!("{}", is_sorted(a))));
    let b: Vec<i64> = vec![1_i64, 3_i64, 2_i64, 4_i64];
    println!("{}", format!("is_sorted([1,3,2,4]) = {}", format!("{}", is_sorted(b))));
    let c: Vec<i64> = vec![1_i64, 2_i64, 2_i64, 3_i64];
    println!("{}", format!("is_sorted([1,2,2,3]) = {}", format!("{}", is_sorted(c))));
    let d: Vec<i64> = vec![7_i64];
    println!("{}", format!("is_sorted([7]) = {}", format!("{}", is_sorted(d))));
    let e: Vec<i64> = vec![];
    println!("{}", format!("is_sorted([]) = {}", format!("{}", is_sorted(e))));
    let f: Vec<i64> = vec![5_i64, 5_i64, 5_i64, 5_i64];
    println!("{}", format!("is_sorted([5,5,5,5]) = {}", format!("{}", is_sorted(f))));
}

