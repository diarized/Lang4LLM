#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn double(x: i64) -> i64 {
    return x * 2_i64;
}

fn is_even(x: i64) -> bool {
    return x - x / 2_i64 * 2_i64 == 0_i64;
}

fn main() {
    let mut numbers: Vec<i64> = vec![5_i64, 2_i64, 8_i64, 1_i64, 9_i64, 3_i64];
    numbers.sort();
    println!("{}", format!("After sort first: {}", numbers[0_i64 as usize]));
    println!("{}", format!("After sort last: {}", numbers[5_i64 as usize]));
    let mut ascending: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    ascending.reverse();
    println!("{}", format!("After reverse first: {}", ascending[0_i64 as usize]));
    let values: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64];
    let doubled: Vec<i64> = values.into_iter().map(move |x| x * 2_i64).collect::<Vec<_>>();
    println!("{}", format!("map doubled[0]: {}", doubled[0_i64 as usize]));
    println!("{}", format!("map doubled[4]: {}", doubled[4_i64 as usize]));
    let mixed: Vec<i64> = vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64];
    let evens: Vec<i64> = mixed.into_iter().filter(|x| (move |x| x - x / 2_i64 * 2_i64 == 0_i64)(x.clone())).collect::<Vec<_>>();
    println!("{}", format!("filter evens count: {}", evens.len()));
    let empty: Vec<i64> = vec![];
    let nonempty: Vec<i64> = vec![1_i64, 2_i64, 3_i64];
    let e1: bool = empty.is_empty();
    let e2: bool = nonempty.is_empty();
    if e1 {
        println!("{}", "empty array detected".to_string());
    } else {
        println!("{}", "should be empty".to_string());
    };
    if e2 {
        println!("{}", "ERROR: nonempty wrongly detected as empty".to_string());
    } else {
        println!("{}", "nonempty array correctly detected".to_string());
    };
    let left: Vec<i64> = vec![1_i64, 2_i64, 3_i64];
    let right: Vec<i64> = vec![4_i64, 5_i64, 6_i64];
    let combined: Vec<i64> = { let mut __v = left; __v.extend(right); __v };
    println!("{}", format!("concat length: {}", combined.len()));
    println!("{}", format!("concat[5]: {}", combined[5_i64 as usize]));
    let items: Vec<i64> = vec![42_i64, 99_i64, 7_i64];
    let head: i64 = items.first().cloned().unwrap();
    println!("{}", format!("first: {}", head));
    let mut stack: Vec<i64> = vec![10_i64, 20_i64, 30_i64];
    let top: i64 = stack.pop().unwrap();
    println!("{}", format!("pop: {}", top));
    println!("{}", format!("after pop len: {}", stack.len()));
    return ();
}

