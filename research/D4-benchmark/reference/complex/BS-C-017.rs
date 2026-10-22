#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Sorter {
    strategy: String,
}

fn sorter_new(strategy: String) -> Sorter {
    return Sorter { strategy: strategy };
}

fn sorter_set_strategy(s: Sorter, strategy: String) {
    s.strategy = strategy;
}

fn bubble_sort(arr: Vec<i64>) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    let mut i: i64 = 0_i64;
    while i < (arr.len() as i64) {
        res.push(arr[(i) as usize].clone());
        i = i + 1_i64;
    }
    let n: i64 = (res.len() as i64);
    let mut swapped: bool = true;
    while swapped {
        swapped = false;
        let mut j: i64 = 0_i64;
        while j < n - 1_i64 {
            if res[(j) as usize].clone() > res[(j + 1_i64) as usize].clone() {
                let tmp: i64 = res[(j) as usize].clone();
                res[(j) as usize] = res[(j + 1_i64) as usize].clone();
                res[(j + 1_i64) as usize] = tmp;
                swapped = true;
            };
            j = j + 1_i64;
        }
    }
    return res;
}

fn insertion_sort(arr: Vec<i64>) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    let mut i: i64 = 0_i64;
    while i < (arr.len() as i64) {
        res.push(arr[(i) as usize].clone());
        i = i + 1_i64;
    }
    let n: i64 = (res.len() as i64);
    let mut i2: i64 = 1_i64;
    while i2 < n {
        let key: i64 = res[(i2) as usize].clone();
        let mut j: i64 = i2 - 1_i64;
        while j >= 0_i64 && res[(j) as usize].clone() > key {
            res[(j + 1_i64) as usize] = res[(j) as usize].clone();
            j = j - 1_i64;
        }
        res[(j + 1_i64) as usize] = key;
        i2 = i2 + 1_i64;
    }
    return res;
}

fn merge_sorted(left: Vec<i64>, right: Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    let mut li: i64 = 0_i64;
    let mut ri: i64 = 0_i64;
    while li < (left.len() as i64) && ri < (right.len() as i64) {
        if left[(li) as usize].clone() <= right[(ri) as usize].clone() {
            result.push(left[(li) as usize].clone());
            li = li + 1_i64;
        } else {
            result.push(right[(ri) as usize].clone());
            ri = ri + 1_i64;
        };
    }
    while li < (left.len() as i64) {
        result.push(left[(li) as usize].clone());
        li = li + 1_i64;
    }
    while ri < (right.len() as i64) {
        result.push(right[(ri) as usize].clone());
        ri = ri + 1_i64;
    }
    return result;
}

fn merge_sort(arr: Vec<i64>) -> Vec<i64> {
    if (arr.len() as i64) <= 1_i64 {
        return arr;
    };
    let mid: i64 = (arr.len() as i64) / 2_i64;
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    let mut i: i64 = 0_i64;
    while i < mid {
        left.push(arr[(i) as usize].clone());
        i = i + 1_i64;
    }
    while i < (arr.len() as i64) {
        right.push(arr[(i) as usize].clone());
        i = i + 1_i64;
    }
    let sorted_left: Vec<i64> = merge_sort(left.clone());
    let sorted_right: Vec<i64> = merge_sort(right.clone());
    return merge_sorted(sorted_left.clone(), sorted_right.clone());
}

fn sorter_sort(s: Sorter, arr: Vec<i64>) -> Vec<i64> {
    if s.strategy == "bubble".to_string() {
        return bubble_sort(arr.clone());
    };
    if s.strategy == "insertion".to_string() {
        return insertion_sort(arr.clone());
    };
    if s.strategy == "merge".to_string() {
        return merge_sort(arr.clone());
    };
    return arr;
}

fn timed_sort(s: Sorter, arr: Vec<i64>) -> Vec<i64> {
    return sorter_sort(s.clone(), arr.clone());
}

fn arr_to_string(arr: Vec<i64>) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    };
    let mut s: String = "[".to_string();
    let mut i: i64 = 0_i64;
    while i < (arr.len() as i64) {
        if i > 0_i64 {
            s = format!("{}, ", s);
        };
        s = format!("{}{}", s, format!("{}", arr[(i) as usize].clone()));
        i = i + 1_i64;
    }
    s = format!("{}]", s);
    return s;
}

fn main() {
    let input: Vec<i64> = vec![5_i64, 2_i64, 8_i64, 1_i64, 9_i64, 3_i64];
    let mut sorter: Sorter = sorter_new("bubble".to_string());
    println!("{}", format!("bubble([5,2,8,1,9,3]) = {}", arr_to_string(sorter_sort(sorter.clone(), input.clone()))));
    sorter_set_strategy(sorter.clone(), "insertion".to_string());
    println!("{}", format!("insertion([5,2,8,1,9,3]) = {}", arr_to_string(sorter_sort(sorter.clone(), input.clone()))));
    sorter_set_strategy(sorter.clone(), "merge".to_string());
    println!("{}", format!("merge([5,2,8,1,9,3]) = {}", arr_to_string(sorter_sort(sorter.clone(), input.clone()))));
    let empty: Vec<i64> = vec![];
    sorter_set_strategy(sorter.clone(), "bubble".to_string());
    println!("{}", format!("bubble([]) = {}", arr_to_string(sorter_sort(sorter.clone(), empty.clone()))));
    sorter_set_strategy(sorter.clone(), "insertion".to_string());
    println!("{}", format!("insertion([]) = {}", arr_to_string(sorter_sort(sorter.clone(), empty.clone()))));
    sorter_set_strategy(sorter.clone(), "merge".to_string());
    println!("{}", format!("merge([]) = {}", arr_to_string(sorter_sort(sorter.clone(), empty.clone()))));
    let single: Vec<i64> = vec![42_i64];
    sorter_set_strategy(sorter.clone(), "bubble".to_string());
    println!("{}", format!("bubble([42]) = {}", arr_to_string(sorter_sort(sorter.clone(), single.clone()))));
    let sorted_in: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    sorter_set_strategy(sorter.clone(), "insertion".to_string());
    println!("{}", format!("insertion(sorted) = {}", arr_to_string(sorter_sort(sorter.clone(), sorted_in.clone()))));
    let rev: Vec<i64> = vec![5_i64, 4_i64, 3_i64, 2_i64, 1_i64];
    sorter_set_strategy(sorter.clone(), "bubble".to_string());
    println!("{}", format!("bubble(reversed) = {}", arr_to_string(sorter_sort(sorter.clone(), rev.clone()))));
    let dups: Vec<i64> = vec![3_i64, 1_i64, 4_i64, 1_i64, 5_i64, 9_i64, 2_i64, 6_i64, 5_i64];
    sorter_set_strategy(sorter.clone(), "merge".to_string());
    println!("{}", format!("merge(dups) = {}", arr_to_string(sorter_sort(sorter.clone(), dups.clone()))));
    let orig: Vec<i64> = vec![7_i64, 3_i64, 5_i64];
    sorter_set_strategy(sorter.clone(), "bubble".to_string());
    let result: Vec<i64> = sorter_sort(sorter.clone(), orig.clone());
    println!("{}", format!("sorted = {}", arr_to_string(result.clone())));
    println!("{}", format!("original = {}", arr_to_string(orig.clone())));
    let tdata: Vec<i64> = vec![10_i64, 5_i64, 3_i64, 8_i64, 1_i64];
    sorter_set_strategy(sorter.clone(), "merge".to_string());
    let tresult: Vec<i64> = timed_sort(sorter.clone(), tdata.clone());
    println!("{}", format!("timed merge([10,5,3,8,1]) = {}", arr_to_string(tresult.clone())));
}

