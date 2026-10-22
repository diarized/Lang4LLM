#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn count_vowels(text: String) -> i64 {
    let vowels: String = "aeiouAEIOU".to_string();
    let char_arr: Vec<String> = text.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut count: i64 = 0_i64;
    for i in 0_i64..char_arr.len() {
        if vowels.contains(&char_arr[i as usize]) {
            count = count + 1_i64;
        };
    }
    return count;
}

fn main() {
    println!("{}", format!("count_vowels("hello") = {}", format!("{}", count_vowels("hello".to_string()))));
    println!("{}", format!("count_vowels("Hello World") = {}", format!("{}", count_vowels("Hello World".to_string()))));
    println!("{}", format!("count_vowels("rhythm") = {}", format!("{}", count_vowels("rhythm".to_string()))));
    println!("{}", format!("count_vowels("aeiou") = {}", format!("{}", count_vowels("aeiou".to_string()))));
    println!("{}", format!("count_vowels("hello123!!") = {}", format!("{}", count_vowels("hello123!!".to_string()))));
    println!("{}", format!("count_vowels("") = {}", format!("{}", count_vowels("".to_string()))));
    println!("{}", format!("count_vowels("AEIOU") = {}", format!("{}", count_vowels("AEIOU".to_string()))));
}

