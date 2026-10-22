#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_digit(c: String) -> bool {
    return c == "0".to_string() || c == "1".to_string() || c == "2".to_string() || c == "3".to_string() || c == "4".to_string() || c == "5".to_string() || c == "6".to_string() || c == "7".to_string() || c == "8".to_string() || c == "9".to_string();
}

fn char_to_digit(c: String) -> i64 {
    if c == "0".to_string() {
        return 0_i64;
    };
    if c == "1".to_string() {
        return 1_i64;
    };
    if c == "2".to_string() {
        return 2_i64;
    };
    if c == "3".to_string() {
        return 3_i64;
    };
    if c == "4".to_string() {
        return 4_i64;
    };
    if c == "5".to_string() {
        return 5_i64;
    };
    if c == "6".to_string() {
        return 6_i64;
    };
    if c == "7".to_string() {
        return 7_i64;
    };
    if c == "8".to_string() {
        return 8_i64;
    };
    return 9_i64;
}

fn encode(text: String) -> String {
    let char_arr: Vec<String> = text.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let n: i64 = (char_arr.len() as i64);
    if n == 0_i64 {
        return "".to_string();
    };
    let mut result: String = "".to_string();
    let mut cur_char: String = char_arr[(0_i64) as usize].clone();
    let mut count: i64 = 1_i64;
    for i in 1_i64..n {
        if char_arr[(i) as usize].clone() == cur_char {
            count = count + 1_i64;
        } else {
            result = format!("{}{}{}", result, format!("{}", count), cur_char);
            cur_char = char_arr[(i) as usize].clone();
            count = 1_i64;
        };
    }
    result = format!("{}{}{}", result, format!("{}", count), cur_char);
    return result;
}

fn decode(encoded: String) -> String {
    let char_arr: Vec<String> = encoded.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let n: i64 = (char_arr.len() as i64);
    if n == 0_i64 {
        return "".to_string();
    };
    let mut result: String = "".to_string();
    let mut pos: i64 = 0_i64;
    while pos < n {
        let mut count: i64 = 0_i64;
        let mut dig_end: i64 = pos.clone();
        let mut scanning: bool = true;
        while scanning {
            if dig_end < n {
                if is_digit(char_arr[(dig_end) as usize].clone()) {
                    count = count * 10_i64 + char_to_digit(char_arr[(dig_end) as usize].clone());
                    dig_end = dig_end + 1_i64;
                } else {
                    scanning = false;
                };
            } else {
                scanning = false;
            };
        }
        let ch: String = char_arr[(dig_end) as usize].clone();
        for k in 0_i64..count {
            result = result + ch;
        }
        pos = dig_end + 1_i64;
    }
    return result;
}

fn main() {
    println!("{}", format!("encode(\"aaabbc\") = \"{}\"", encode("aaabbc".to_string())));
    println!("{}", format!("encode(\"abcde\") = \"{}\"", encode("abcde".to_string())));
    println!("{}", format!("encode(\"aaaaaaa\") = \"{}\"", encode("aaaaaaa".to_string())));
    println!("{}", format!("decode(\"3a2b1c\") = \"{}\"", decode("3a2b1c".to_string())));
    println!("{}", format!("decode(\"1a1b1c1d1e\") = \"{}\"", decode("1a1b1c1d1e".to_string())));
    let original: String = "aabbccddee".to_string();
    let encoded_rt: String = encode(original.clone());
    println!("{}", format!("encode(\"aabbccddee\") = \"{}\"", encoded_rt));
    println!("{}", format!("decode(encode(\"aabbccddee\")) = \"{}\"", decode(encoded_rt.clone())));
    println!("{}", format!("encode(\"\") = \"{}\"", encode("".to_string())));
    println!("{}", format!("decode(\"\") = \"{}\"", decode("".to_string())));
    println!("{}", format!("encode(\"z\") = \"{}\"", encode("z".to_string())));
    println!("{}", format!("encode(\"111222\") = \"{}\"", encode("111222".to_string())));
    println!("{}", format!("decode(\"12a3b\") = \"{}\"", decode("12a3b".to_string())));
}

