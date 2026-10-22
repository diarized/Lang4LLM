#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn lower_letters() -> Vec<String> {
    return vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(), "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(), "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(), "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()];
}

fn upper_letters() -> Vec<String> {
    return vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(), "I".to_string(), "J".to_string(), "K".to_string(), "L".to_string(), "M".to_string(), "N".to_string(), "O".to_string(), "P".to_string(), "Q".to_string(), "R".to_string(), "S".to_string(), "T".to_string(), "U".to_string(), "V".to_string(), "W".to_string(), "X".to_string(), "Y".to_string(), "Z".to_string()];
}

fn find_letter_index(c: String, letters: Vec<String>) -> i64 {
    let mut idx: i64 = -1_i64;
    for i in 0_i64..26_i64 {
        if letters[(i) as usize].clone() == c {
            idx = i;
        };
    }
    return idx;
}

fn normalize_shift(s: i64) -> i64 {
    let r: i64 = s - s / 26_i64 * 26_i64;
    if r < 0_i64 {
        return r + 26_i64;
    };
    return r;
}

fn caesar_encode(text: String, shift: i64) -> String {
    let ns: i64 = normalize_shift(shift.clone());
    let lowers: Vec<String> = lower_letters();
    let uppers: Vec<String> = upper_letters();
    let chs: Vec<String> = text.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut result: String = "".to_string();
    for i in 0_i64..(chs.len() as i64) {
        let c: String = chs[(i) as usize].clone();
        let li: i64 = find_letter_index(c.clone(), lowers.clone());
        let ui: i64 = find_letter_index(c.clone(), uppers.clone());
        if li >= 0_i64 {
            let new_idx: i64 = li + ns - (li + ns) / 26_i64 * 26_i64;
            result = result + lowers[(new_idx) as usize].clone();
        } else {
            if ui >= 0_i64 {
                let new_idx2: i64 = ui + ns - (ui + ns) / 26_i64 * 26_i64;
                result = result + uppers[(new_idx2) as usize].clone();
            } else {
                result = result + c;
            };
        };
    }
    return result;
}

fn main() {
    println!("{}", format!("caesar_encode(Hello, 3) = {}", caesar_encode("Hello".to_string(), 3_i64)));
    println!("{}", format!("caesar_encode(xyz, 3) = {}", caesar_encode("xyz".to_string(), 3_i64)));
    println!("{}", format!("caesar_encode(XYZ, 3) = {}", caesar_encode("XYZ".to_string(), 3_i64)));
    println!("{}", format!("caesar_encode(Hello World 123, 13) = {}", caesar_encode("Hello, World! 123".to_string(), 13_i64)));
    println!("{}", format!("caesar_encode(Khoor, -3) = {}", caesar_encode("Khoor".to_string(), -3_i64)));
    println!("{}", format!("caesar_encode(NoChange, 0) = {}", caesar_encode("NoChange".to_string(), 0_i64)));
    println!("{}", format!("caesar_encode(abc, 29) = {}", caesar_encode("abc".to_string(), 29_i64)));
    println!("{}", format!("caesar_encode(Hello, 26) = {}", caesar_encode("Hello".to_string(), 26_i64)));
    println!("{}", format!("caesar_encode(, 5) = {}", caesar_encode("".to_string(), 5_i64)));
    println!("{}", format!("caesar_encode(123!?, 7) = {}", caesar_encode("123!?".to_string(), 7_i64)));
    println!("{}", format!("caesar_encode(Zebra, 2) = {}", caesar_encode("Zebra".to_string(), 2_i64)));
}

