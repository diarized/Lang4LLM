#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn stat_mean(nums: Vec<f64>) -> f64 {
    let n: i64 = (nums.len() as i64);
    if n == 0_i64 {
        return 0.0;
    };
    let mut total: f64 = 0.0;
    for i in 0_i64..n {
        total = total + nums[(i) as usize].clone();
    }
    return total / { let __sf = format!("{}", n); __sf.parse::<f64>().unwrap_or(0.0) };
}

fn stat_median(nums: Vec<f64>) -> f64 {
    let n: i64 = (nums.len() as i64);
    if n == 0_i64 {
        return 0.0;
    };
    let mut sorted_nums: Vec<f64> = vec![];
    for i in 0_i64..n {
        sorted_nums.push(nums[(i) as usize].clone());
    }
    sorted_nums.sort();
    if n - n / 2_i64 * 2_i64 == 1_i64 {
        return sorted_nums[(n / 2_i64) as usize].clone();
    } else {
        let mid1: f64 = sorted_nums[(n / 2_i64 - 1_i64) as usize].clone();
        let mid2: f64 = sorted_nums[(n / 2_i64) as usize].clone();
        return (mid1 + mid2) / 2.0;
    };
}

fn stat_mode(nums: Vec<i64>) -> i64 {
    let n: i64 = (nums.len() as i64);
    if n == 0_i64 {
        return 0_i64;
    };
    let mut sorted_mode: Vec<i64> = vec![];
    for i in 0_i64..n {
        sorted_mode.push(nums[(i) as usize].clone());
    }
    sorted_mode.sort();
    let mut best_val: i64 = sorted_mode[(0_i64) as usize].clone();
    let mut best_count: i64 = 1_i64;
    let mut cur_val: i64 = sorted_mode[(0_i64) as usize].clone();
    let mut cur_count: i64 = 1_i64;
    for i in 1_i64..n {
        if sorted_mode[(i) as usize].clone() == cur_val {
            cur_count = cur_count + 1_i64;
        } else {
            if cur_count > best_count {
                best_count = cur_count;
                best_val = cur_val;
            };
            cur_val = sorted_mode[(i) as usize].clone();
            cur_count = 1_i64;
        };
    }
    if cur_count > best_count {
        best_val = cur_val;
    };
    return best_val;
}

fn main() {
    let n1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let m1: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    println!("{}", format!("mean([1,2,3,4,5]) = {}", format!("{}", stat_mean(n1.clone()))));
    println!("{}", format!("median([1,2,3,4,5]) = {}", format!("{}", stat_median(n1.clone()))));
    println!("{}", format!("mode([1,2,3,4,5]) = {}", format!("{}", stat_mode(m1.clone()))));
    let n2: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0];
    let m2: Vec<i64> = vec![2_i64, 4_i64, 6_i64, 8_i64];
    println!("{}", format!("mean([2,4,6,8]) = {}", format!("{}", stat_mean(n2.clone()))));
    println!("{}", format!("median([2,4,6,8]) = {}", format!("{}", stat_median(n2.clone()))));
    println!("{}", format!("mode([2,4,6,8]) = {}", format!("{}", stat_mode(m2.clone()))));
    let m3: Vec<i64> = vec![1_i64, 2_i64, 2_i64, 3_i64, 4_i64];
    println!("{}", format!("mode([1,2,2,3,4]) = {}", format!("{}", stat_mode(m3.clone()))));
    let m4: Vec<i64> = vec![1_i64, 1_i64, 2_i64, 2_i64, 3_i64];
    println!("{}", format!("mode([1,1,2,2,3]) = {}", format!("{}", stat_mode(m4.clone()))));
    let n5: Vec<f64> = vec![42.0];
    let m5: Vec<i64> = vec![42_i64];
    println!("{}", format!("mean([42]) = {}", format!("{}", stat_mean(n5.clone()))));
    println!("{}", format!("median([42]) = {}", format!("{}", stat_median(n5.clone()))));
    println!("{}", format!("mode([42]) = {}", format!("{}", stat_mode(m5.clone()))));
    let n6: Vec<f64> = vec![5.0, 5.0, 5.0, 5.0];
    let m6: Vec<i64> = vec![5_i64, 5_i64, 5_i64, 5_i64];
    println!("{}", format!("mean([5,5,5,5]) = {}", format!("{}", stat_mean(n6.clone()))));
    println!("{}", format!("median([5,5,5,5]) = {}", format!("{}", stat_median(n6.clone()))));
    println!("{}", format!("mode([5,5,5,5]) = {}", format!("{}", stat_mode(m6.clone()))));
    let n7: Vec<f64> = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    let m7: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64, 9_i64, 2_i64, 6_i64];
    println!("{}", format!("median([3,1,4,1,5,9,2,6]) = {}", format!("{}", stat_median(n7.clone()))));
    println!("{}", format!("mode([3,1,4,1,5,9,2,6]) = {}", format!("{}", stat_mode(m7.clone()))));
    let m8: Vec<i64> = vec![-3_i64, -1_i64, -2_i64];
    println!("{}", format!("mode([-3,-1,-2]) = {}", format!("{}", stat_mode(m8.clone()))));
    let m9: Vec<i64> = vec![3_i64, 3_i64, 3_i64, 1_i64, 2_i64];
    println!("{}", format!("mode([3,3,3,1,2]) = {}", format!("{}", stat_mode(m9.clone()))));
    let n10: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    println!("{}", format!("median([1,2,3,4,5,6]) = {}", format!("{}", stat_median(n10.clone()))));
}

