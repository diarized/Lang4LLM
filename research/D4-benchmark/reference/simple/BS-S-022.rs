#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn count_words(text: String) -> i64 {
    let trimmed: String = text.trim().to_string();
    if trimmed.len() as i64 == 0_i64 {
        return 0_i64;
    };
    let parts: Vec<String> = trimmed.split(" ".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    let mut count: i64 = 0_i64;
    for i in 0_i64..parts.len() as i64 {
        if parts[(i) as usize].clone().trim().to_string().len() as i64 > 0_i64 {
            count = count + 1_i64;
        };
    }
    return count;
}

fn main() {
    println!("{}", format!("{}", count_words("hello world".to_string())));
    println!("{}", format!("{}", count_words("hello   world".to_string())));
    println!("{}", format!("{}", count_words("  hello world  ".to_string())));
    println!("{}", format!("{}", count_words("".to_string())));
    println!("{}", format!("{}", count_words("hello".to_string())));
    println!("{}", format!("{}", count_words("   ".to_string())));
    println!("{}", format!("{}", count_words("one two three four".to_string())));
    println!("{}", format!("{}", count_words("  leading and trailing  ".to_string())));
}

