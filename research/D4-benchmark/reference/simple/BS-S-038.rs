#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn clamp(value: i64, min_val: i64, max_val: i64) -> i64 {
    if value < min_val {
        return min_val;
    };
    if value > max_val {
        return max_val;
    };
    return value;
}

fn clamp_float(value: f64, min_val: f64, max_val: f64) -> f64 {
    if value < min_val {
        return min_val;
    };
    if value > max_val {
        return max_val;
    };
    return value;
}

fn main() {
    println!("{}", format!("{}", clamp(5_i64, 1_i64, 10_i64)));
    println!("{}", format!("{}", clamp(-3_i64, 0_i64, 100_i64)));
    println!("{}", format!("{}", clamp(150_i64, 0_i64, 100_i64)));
    println!("{}", format!("{}", clamp(0_i64, 0_i64, 10_i64)));
    println!("{}", format!("{}", clamp(10_i64, 0_i64, 10_i64)));
    println!("{}", format!("{}", clamp_float(1.5, 0.0, 1.0)));
    println!("{}", format!("{}", clamp(-5_i64, -10_i64, -1_i64)));
}

