#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_prime(number: i64) -> bool {
    if number < 2_i64 {
        return false;
    };
    if number == 2_i64 {
        return true;
    };
    if number % 2_i64 == 0_i64 {
        return false;
    };
    let limit: i64 = { let __s = format!("{}", ({ let __sf = format!("{}", number); __sf.parse::<f64>().unwrap_or(0.0) }).sqrt()); __s.parse::<i64>().unwrap_or(0) };
    let mut i: i64 = 3_i64;
    let mut found_factor: bool = false;
    for j in 0_i64..1000000_i64 {
        if i <= limit {
            if number % i == 0_i64 {
                found_factor = true;
            };
            i = i + 2_i64;
        };
    }
    return found_factor == false;
}

fn main() {
    println!("{}", format!("{}", is_prime(7_i64)));
    println!("{}", format!("{}", is_prime(9_i64)));
    println!("{}", format!("{}", is_prime(1_i64)));
    println!("{}", format!("{}", is_prime(2_i64)));
    println!("{}", format!("{}", is_prime(4_i64)));
    println!("{}", format!("{}", is_prime(97_i64)));
    println!("{}", format!("{}", is_prime(100_i64)));
    println!("{}", format!("{}", is_prime(3_i64)));
}

