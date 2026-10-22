#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn find_index(items: Vec<i64>, target: i64) -> i64 {
    for i in 0_i64..items.len() as i64 {
        if items[(i) as usize].clone() == target {
            return i;
        };
    }
    return -1_i64;
}

fn main() {
    let a1: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64];
    println!("{}", format!("{}", find_index(a1, 3_i64)));
    let a2: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64];
    println!("{}", format!("{}", find_index(a2, 4_i64)));
    let a3: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64];
    println!("{}", format!("{}", find_index(a3, 7_i64)));
    let a4: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64];
    println!("{}", format!("{}", find_index(a4, 1_i64)));
    let empty: Vec<i64> = vec![];
    println!("{}", format!("{}", find_index(empty, 5_i64)));
    let s1: Vec<i64> = vec![42_i64];
    println!("{}", format!("{}", find_index(s1, 42_i64)));
    let s2: Vec<i64> = vec![42_i64];
    println!("{}", format!("{}", find_index(s2, 7_i64)));
    let b: Vec<i64> = vec![10_i64, 20_i64, 30_i64, 40_i64, 50_i64];
    println!("{}", format!("{}", find_index(b, 50_i64)));
}

