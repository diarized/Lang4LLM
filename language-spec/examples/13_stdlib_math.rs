#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let neg: i64 = 0_i64 - 5_i64;
    let pos: i64 = (neg).abs();
    println!("{}", format!("abs(0 - 5) = {}", pos));
    let smaller: i64 = std::cmp::min(3_i64, 7_i64);
    let larger: i64 = std::cmp::max(3_i64, 7_i64);
    println!("{}", format!("min(3, 7) = {}", smaller));
    println!("{}", format!("max(3, 7) = {}", larger));
    let result: i64 = (2_i64).pow(10_i64 as u32);
    println!("{}", format!("pow(2, 10) = {}", result));
    let x: f64 = 3.7;
    let floored: f64 = (x).floor();
    let ceiled: f64 = (x).ceil();
    let rounded: f64 = (x).round();
    println!("{}", format!("floor(3.7) = {}", floored));
    println!("{}", format!("ceil(3.7) = {}", ceiled));
    println!("{}", format!("round(3.7) = {}", rounded));
    let n: i64 = 9_i64;
    let f: f64 = (n as f64);
    let root: f64 = (f).sqrt();
    println!("{}", format!("sqrt(9) = {}", root));
    let num_str: String = format!("{}", 42_i64);
    println!("{}", format!("to_string(42) = {}", num_str));
    return ();
}

