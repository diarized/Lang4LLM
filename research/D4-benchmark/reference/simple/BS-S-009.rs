#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn count_occurrences(numbers: Vec<i64>, target: i64) -> i64 {
    let mut count: i64 = 0_i64;
    for i in 0_i64..numbers.len() {
        if numbers[i as usize] == target {
            count = count + 1_i64;
        };
    }
    return count;
}

fn main() {
    let a: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 2_i64, 4_i64, 2_i64];
    println!("{}", format!("count_occurrences([1,2,3,2,4,2], 2) = {}", format!("{}", count_occurrences(a, 2_i64))));
    let b: Vec<i64> = vec![5_i64, 10_i64, 15_i64, 20_i64];
    println!("{}", format!("count_occurrences([5,10,15,20], 15) = {}", format!("{}", count_occurrences(b, 15_i64))));
    let c: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64];
    println!("{}", format!("count_occurrences([1,2,3,4], 9) = {}", format!("{}", count_occurrences(c, 9_i64))));
    let d: Vec<i64> = vec![];
    println!("{}", format!("count_occurrences([], 5) = {}", format!("{}", count_occurrences(d, 5_i64))));
    let e: Vec<i64> = vec![3_i64, 3_i64, 3_i64, 3_i64];
    println!("{}", format!("count_occurrences([3,3,3,3], 3) = {}", format!("{}", count_occurrences(e, 3_i64))));
    let f: Vec<i64> = vec![-2_i64, 0_i64, -2_i64, 1_i64];
    println!("{}", format!("count_occurrences([-2,0,-2,1], -2) = {}", format!("{}", count_occurrences(f, -2_i64))));
}

