#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn find_second_max_str(numbers: Vec<i64>) -> String {
    let n: i64 = numbers.len();
    if n == 0_i64 {
        return "none".to_string();
    };
    let mut max1: i64 = numbers[0_i64 as usize];
    for i in 1_i64..n {
        if numbers[i as usize] > max1 {
            max1 = numbers[i as usize];
        };
    }
    let mut found_second: bool = false;
    let mut max2: i64 = 0_i64;
    for i in 0_i64..n {
        if numbers[i as usize] < max1 {
            if found_second == false {
                max2 = numbers[i as usize];
                found_second = true;
            } else {
                if numbers[i as usize] > max2 {
                    max2 = numbers[i as usize];
                };
            };
        };
    }
    if found_second == false {
        return "none".to_string();
    };
    return format!("{}", max2);
}

fn main() {
    let a: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64, 9_i64, 2_i64, 6_i64];
    println!("{}", format!("find_second_max([3,1,4,1,5,9,2,6]) = {}", find_second_max_str(a)));
    let b: Vec<i64> = vec![5_i64, 5_i64, 5_i64];
    println!("{}", format!("find_second_max([5,5,5]) = {}", find_second_max_str(b)));
    let c: Vec<i64> = vec![4_i64, 4_i64, 7_i64, 7_i64];
    println!("{}", format!("find_second_max([4,4,7,7]) = {}", find_second_max_str(c)));
    let d: Vec<i64> = vec![10_i64];
    println!("{}", format!("find_second_max([10]) = {}", find_second_max_str(d)));
    let e: Vec<i64> = vec![-1_i64, -5_i64, -3_i64];
    println!("{}", format!("find_second_max([-1,-5,-3]) = {}", find_second_max_str(e)));
    let f: Vec<i64> = vec![1_i64, 2_i64];
    println!("{}", format!("find_second_max([1,2]) = {}", find_second_max_str(f)));
}

