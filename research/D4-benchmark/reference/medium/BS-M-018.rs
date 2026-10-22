#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn arr_to_str(arr: Vec<i64>) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    };
    let mut s: String = "[".to_string();
    for i in 0_i64..(arr.len() as i64) {
        if i > 0_i64 {
            s = format!("{}, ", s);
        };
        s = format!("{}{}", s, format!("{}", arr[(i) as usize].clone()));
    }
    s = format!("{}]", s);
    return s;
}

fn perm_helper(nums: Vec<i64>, chosen: Vec<i64>, used: Vec<bool>, results: Vec<String>) -> Vec<String> {
    let n: i64 = (nums.len() as i64);
    if (chosen.len() as i64) == n {
        let mut new_results: Vec<String> = results.clone();
        new_results.push(arr_to_str(chosen.clone()));
        return new_results;
    };
    let mut cur_results: Vec<String> = results.clone();
    for i in 0_i64..n {
        if !used[(i) as usize].clone() {
            let mut new_chosen: Vec<i64> = chosen.clone();
            new_chosen.push(nums[(i) as usize].clone());
            let mut new_used: Vec<bool> = used.clone();
            new_used[(i) as usize] = true;
            cur_results = perm_helper(nums.clone(), new_chosen.clone(), new_used.clone(), cur_results.clone());
        };
    }
    return cur_results;
}

fn permutations(nums: Vec<i64>) -> Vec<String> {
    if nums.is_empty() {
        let mut empty_result: Vec<String> = vec![];
        empty_result.push("[]".to_string());
        return empty_result;
    };
    let mut sorted_nums: Vec<i64> = nums.clone();
    sorted_nums.sort();
    let n: i64 = (sorted_nums.len() as i64);
    let mut used: Vec<bool> = vec![];
    for i in 0_i64..n {
        used.push(false);
    }
    let chosen: Vec<i64> = vec![];
    let base: Vec<String> = vec![];
    let raw: Vec<String> = perm_helper(sorted_nums.clone(), chosen.clone(), used.clone(), base.clone());
    raw.sort();
    return raw;
}

fn main() {
    let p1: Vec<String> = permutations(vec![1_i64, 2_i64, 3_i64]);
    println!("{}", format!("count([1,2,3]) = {}", format!("{}", (p1.len() as i64))));
    for perm in p1 {
        println!("{}", perm);
    }
    let p2: Vec<String> = permutations(vec![1_i64, 2_i64]);
    println!("{}", format!("count([1,2]) = {}", format!("{}", (p2.len() as i64))));
    for perm2 in p2 {
        println!("{}", perm2);
    }
    let p3: Vec<String> = permutations(vec![5_i64]);
    println!("{}", format!("count([5]) = {}", format!("{}", (p3.len() as i64))));
    println!("{}", p3[(0_i64) as usize].clone());
    let p4: Vec<String> = permutations(vec![]);
    println!("{}", format!("count([]) = {}", format!("{}", (p4.len() as i64))));
    println!("{}", p4[(0_i64) as usize].clone());
    let p5: Vec<String> = permutations(vec![3_i64, 1_i64, 2_i64]);
    println!("{}", format!("count([3,1,2]) = {}", format!("{}", (p5.len() as i64))));
    println!("{}", format!("first perm: {}", p5[(0_i64) as usize].clone()));
    println!("{}", format!("last perm: {}", p5[(5_i64) as usize].clone()));
}

