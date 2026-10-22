#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn count_digits(number: i64) -> i64 {
    if number == 0_i64 {
        return 1_i64;
    };
    let mut n: i64 = number;
    let mut count: i64 = 0_i64;
    for i in 0_i64..20_i64 {
        if n > 0_i64 {
            count = count + 1_i64;
            n = n / 10_i64;
        };
    }
    return count;
}

fn main() {
    println!("{}", format!("{}", count_digits(5_i64)));
    println!("{}", format!("{}", count_digits(42_i64)));
    println!("{}", format!("{}", count_digits(100_i64)));
    println!("{}", format!("{}", count_digits(0_i64)));
    println!("{}", format!("{}", count_digits(1000000_i64)));
    println!("{}", format!("{}", count_digits(9_i64)));
    println!("{}", format!("{}", count_digits(111_i64)));
}

