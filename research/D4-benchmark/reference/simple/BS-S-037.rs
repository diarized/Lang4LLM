#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn fizzbuzz(n: i64) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for i in 1_i64..n + 1_i64 {
        if i % 15_i64 == 0_i64 {
            result.push("FizzBuzz".to_string());
        } else {
            if i % 3_i64 == 0_i64 {
                result.push("Fizz".to_string());
            } else {
                if i % 5_i64 == 0_i64 {
                    result.push("Buzz".to_string());
                } else {
                    result.push(format!("{}", i));
                };
            };
        };
    }
    return result;
}

fn main() {
    let r15: Vec<String> = fizzbuzz(15_i64);
    println!("{}", r15.join(", ".to_string().as_str()));
    let r1: Vec<String> = fizzbuzz(1_i64);
    println!("{}", r1.join(", ".to_string().as_str()));
    let r5: Vec<String> = fizzbuzz(5_i64);
    println!("{}", r5.join(", ".to_string().as_str()));
    let r3: Vec<String> = fizzbuzz(3_i64);
    println!("{}", r3.join(", ".to_string().as_str()));
}

