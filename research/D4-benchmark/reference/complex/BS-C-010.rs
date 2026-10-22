#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_digit(c: String) -> bool {
    return c == "0".to_string() || c == "1".to_string() || c == "2".to_string() || c == "3".to_string() || c == "4".to_string() || c == "5".to_string() || c == "6".to_string() || c == "7".to_string() || c == "8".to_string() || c == "9".to_string();
}

fn is_whitespace(c: String) -> bool {
    return c == " ".to_string() || c == "\t".to_string();
}

fn tokenize(expr: String) -> Vec<String> {
    let chs: Vec<String> = expr.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut tokens: Vec<String> = vec![];
    let mut i: i64 = 0_i64;
    let mut last_type: String = "none".to_string();
    let mut had_error: bool = false;
    while i < (chs.len() as i64) && !had_error {
        let c: String = chs[(i) as usize].clone();
        if is_whitespace(c.clone()) {
            i = i + 1_i64;
        } else {
            if is_digit(c.clone()) {
                let mut num: String = "".to_string();
                let mut dot_count: i64 = 0_i64;
                while i < (chs.len() as i64) && (is_digit(chs[(i) as usize].clone()) || chs[(i) as usize].clone() == ".".to_string()) {
                    if chs[(i) as usize].clone() == ".".to_string() {
                        dot_count = dot_count + 1_i64;
                        if dot_count > 1_i64 {
                            tokens.push("ERROR:multiple_dots".to_string());
                            had_error = true;
                            i = (chs.len() as i64);
                        } else {
                            num = format!("{}{}", num, format!("{}", chs[(i) as usize].clone()));
                            i = i + 1_i64;
                        };
                    } else {
                        num = format!("{}{}", num, format!("{}", chs[(i) as usize].clone()));
                        i = i + 1_i64;
                    };
                }
                if !had_error {
                    if num.ends_with(".".to_string().as_str()) {
                        tokens.push("ERROR:trailing_dot".to_string());
                        had_error = true;
                    } else {
                        tokens.push(format!("NUM:{}", num));
                        last_type = "NUM".to_string();
                    };
                };
            } else {
                if c == "+".to_string() {
                    tokens.push("PLUS".to_string());
                    last_type = "PLUS".to_string();
                    i = i + 1_i64;
                } else {
                    if c == "-".to_string() {
                        if last_type == "NUM".to_string() || last_type == "RPAREN".to_string() {
                            tokens.push("MINUS".to_string());
                            last_type = "MINUS".to_string();
                        } else {
                            tokens.push("NEGATE".to_string());
                            last_type = "NEGATE".to_string();
                        };
                        i = i + 1_i64;
                    } else {
                        if c == "*".to_string() {
                            tokens.push("MUL".to_string());
                            last_type = "MUL".to_string();
                            i = i + 1_i64;
                        } else {
                            if c == "/".to_string() {
                                tokens.push("DIV".to_string());
                                last_type = "DIV".to_string();
                                i = i + 1_i64;
                            } else {
                                if c == "(".to_string() {
                                    tokens.push("LPAREN".to_string());
                                    last_type = "LPAREN".to_string();
                                    i = i + 1_i64;
                                } else {
                                    if c == ")".to_string() {
                                        tokens.push("RPAREN".to_string());
                                        last_type = "RPAREN".to_string();
                                        i = i + 1_i64;
                                    } else {
                                        if c == ".".to_string() {
                                            tokens.push("ERROR:leading_dot".to_string());
                                            had_error = true;
                                            i = (chs.len() as i64);
                                        } else {
                                            tokens.push("ERROR:unknown_char".to_string());
                                            had_error = true;
                                            i = (chs.len() as i64);
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
    return tokens;
}

fn print_tokens(expr: String) {
    let toks: Vec<String> = tokenize(expr.clone());
    let mut i: i64 = 0_i64;
    while i < (toks.len() as i64) {
        println!("{}", toks[(i) as usize].clone());
        i = i + 1_i64;
    }
}

fn main() {
    println!("{}", "--- 3 + 4 * 2 ---".to_string());
    print_tokens("3 + 4 * 2".to_string());
    println!("{}", "--- 3.14 * 2 ---".to_string());
    print_tokens("3.14 * 2".to_string());
    println!("{}", "--- -3 + 5 - 2 ---".to_string());
    print_tokens("-3 + 5 - 2".to_string());
    println!("{}", "--- (-3 + -4) ---".to_string());
    print_tokens("(-3 + -4)".to_string());
    println!("{}", "--- ((1 + 2)) / 3 ---".to_string());
    print_tokens("((1 + 2)) / 3".to_string());
    println!("{}", "--- empty ---".to_string());
    print_tokens("".to_string());
    println!("{}", "--- 3 @ 4 ---".to_string());
    print_tokens("3 @ 4".to_string());
    println!("{}", "--- 5 - -3 ---".to_string());
    print_tokens("5 - -3".to_string());
}

