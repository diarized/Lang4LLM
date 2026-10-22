#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn gcd(a: i64, b: i64) -> i64 {
    let mut x: i64 = a;
    let mut y: i64 = b;
    let mut running: bool = true;
    for i in 0_i64..1000000_i64 {
        if running {
            if y == 0_i64 {
                running = false;
            } else {
                let temp: i64 = y;
                y = x % y;
                x = temp;
            };
        };
    }
    return x;
}

fn main() {
    println!("{}", format!("{}", gcd(48_i64, 18_i64)));
    println!("{}", format!("{}", gcd(12_i64, 4_i64)));
    println!("{}", format!("{}", gcd(7_i64, 13_i64)));
    println!("{}", format!("{}", gcd(9_i64, 9_i64)));
    println!("{}", format!("{}", gcd(1_i64, 100_i64)));
    println!("{}", format!("{}", gcd(100_i64, 1_i64)));
    println!("{}", format!("{}", gcd(1000000_i64, 500000_i64)));
}

