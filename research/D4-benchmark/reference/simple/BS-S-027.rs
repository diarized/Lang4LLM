#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn my_abs(number: i64) -> i64 {
    if number < 0_i64 {
        return 0_i64 - number;
    };
    return number;
}

fn main() {
    println!("{}", format!("{}", my_abs(-7_i64)));
    println!("{}", format!("{}", my_abs(5_i64)));
    println!("{}", format!("{}", my_abs(0_i64)));
    println!("{}", format!("{}", my_abs(-1000_i64)));
    println!("{}", format!("{}", my_abs(1000_i64)));
    println!("{}", format!("{}", my_abs(-1_i64)));
    println!("{}", format!("{}", my_abs(1_i64)));
}

