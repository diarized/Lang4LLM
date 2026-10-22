#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn normalize(s: String) -> String {
    let lower_s: String = s.to_lowercase();
    let chs: Vec<String> = lower_s.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut result: String = "".to_string();
    for i in 0_i64..(chs.len() as i64) {
        if chs[(i) as usize].clone() != " ".to_string() {
            result = result + chs[(i) as usize].clone();
        };
    }
    return result;
}

fn char_to_index(c: String) -> i64 {
    let alphabet: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    let alph_chs: Vec<String> = alphabet.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut idx: i64 = -1_i64;
    for i in 0_i64..(alph_chs.len() as i64) {
        if alph_chs[(i) as usize].clone() == c {
            idx = i;
        };
    }
    return idx;
}

fn is_anagram(a: String, b: String) -> bool {
    let na: String = normalize(a.clone());
    let nb: String = normalize(b.clone());
    if (na.len() as i64) != (nb.len() as i64) {
        return false;
    };
    let mut counts: Vec<i64> = vec![];
    for i in 0_i64..26_i64 {
        counts.push(0_i64);
    }
    let chs_a: Vec<String> = na.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    for i in 0_i64..(chs_a.len() as i64) {
        let idx_a: i64 = char_to_index(chs_a[(i) as usize].clone());
        if idx_a >= 0_i64 {
            counts[(idx_a) as usize] = counts[(idx_a) as usize].clone() + 1_i64;
        };
    }
    let chs_b: Vec<String> = nb.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    for i in 0_i64..(chs_b.len() as i64) {
        let idx_b: i64 = char_to_index(chs_b[(i) as usize].clone());
        if idx_b >= 0_i64 {
            counts[(idx_b) as usize] = counts[(idx_b) as usize].clone() - 1_i64;
        };
    }
    let mut all_zero: bool = true;
    for i in 0_i64..26_i64 {
        if counts[(i) as usize].clone() != 0_i64 {
            all_zero = false;
        };
    }
    return all_zero;
}

fn main() {
    println!("{}", format!("is_anagram(listen, silent) = {}", format!("{}", is_anagram("listen".to_string(), "silent".to_string()))));
    println!("{}", format!("is_anagram(Listen, Silent) = {}", format!("{}", is_anagram("Listen".to_string(), "Silent".to_string()))));
    println!("{}", format!("is_anagram(hello, world) = {}", format!("{}", is_anagram("hello".to_string(), "world".to_string()))));
    println!("{}", format!("is_anagram(the morse code, here come dots) = {}", format!("{}", is_anagram("the morse code".to_string(), "here come dots".to_string()))));
    println!("{}", format!("is_anagram(abc, ab) = {}", format!("{}", is_anagram("abc".to_string(), "ab".to_string()))));
    println!("{}", format!("is_anagram(, ) = {}", format!("{}", is_anagram("".to_string(), "".to_string()))));
    println!("{}", format!("is_anagram(, a) = {}", format!("{}", is_anagram("".to_string(), "a".to_string()))));
    println!("{}", format!("is_anagram(aab, bba) = {}", format!("{}", is_anagram("aab".to_string(), "bba".to_string()))));
    println!("{}", format!("is_anagram(a, a) = {}", format!("{}", is_anagram("a".to_string(), "a".to_string()))));
    println!("{}", format!("is_anagram(a, b) = {}", format!("{}", is_anagram("a".to_string(), "b".to_string()))));
}

