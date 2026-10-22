#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_numeric(text: String) -> bool {
    if text.len() as i64 == 0_i64 {
        return false;
    };
    let ch: Vec<String> = text.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut start: i64 = 0_i64;
    if ch[(0_i64) as usize].clone() == "-".to_string() {
        start = 1_i64;
    };
    if start == text.len() as i64 {
        return false;
    };
    let mut valid: bool = true;
    for i in start..text.len() as i64 {
        let c: String = ch[(i) as usize].clone();
        if c == "0".to_string() {
            valid = valid;
        } else {
            if c == "1".to_string() {
                valid = valid;
            } else {
                if c == "2".to_string() {
                    valid = valid;
                } else {
                    if c == "3".to_string() {
                        valid = valid;
                    } else {
                        if c == "4".to_string() {
                            valid = valid;
                        } else {
                            if c == "5".to_string() {
                                valid = valid;
                            } else {
                                if c == "6".to_string() {
                                    valid = valid;
                                } else {
                                    if c == "7".to_string() {
                                        valid = valid;
                                    } else {
                                        if c == "8".to_string() {
                                            valid = valid;
                                        } else {
                                            if c == "9".to_string() {
                                                valid = valid;
                                            } else {
                                                valid = false;
                                            };
                                        };
                                    };
                                };
                            };
                        };
                    };
                };
            };
        };
    }
    return valid;
}

fn main() {
    println!("{}", format!("{}", is_numeric("123".to_string())));
    println!("{}", format!("{}", is_numeric("-456".to_string())));
    println!("{}", format!("{}", is_numeric("12a3".to_string())));
    println!("{}", format!("{}", is_numeric("".to_string())));
    println!("{}", format!("{}", is_numeric("-".to_string())));
    println!("{}", format!("{}", is_numeric(" 123".to_string())));
    println!("{}", format!("{}", is_numeric("3.14".to_string())));
    println!("{}", format!("{}", is_numeric("0".to_string())));
    println!("{}", format!("{}", is_numeric("007".to_string())));
    println!("{}", format!("{}", is_numeric("+5".to_string())));
}

