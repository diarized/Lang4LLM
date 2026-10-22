#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn isqrt(n: i64) -> i64 {
    if n == 0_i64 {
        return 0_i64;
    };
    let mut r: i64 = 0_i64;
    let mut running: bool = true;
    for i in 0_i64..100000_i64 {
        if running {
            let next: i64 = r + 1_i64;
            if next * next <= n {
                r = next;
            } else {
                running = false;
            };
        };
    }
    return r;
}

fn main() {
    println!("{}", format!("{}", isqrt(9_i64)));
    println!("{}", format!("{}", isqrt(8_i64)));
    println!("{}", format!("{}", isqrt(0_i64)));
    println!("{}", format!("{}", isqrt(1_i64)));
    println!("{}", format!("{}", isqrt(100_i64)));
    println!("{}", format!("{}", isqrt(15_i64)));
    println!("{}", format!("{}", isqrt(2_i64)));
    println!("{}", format!("{}", isqrt(10000_i64)));
}

