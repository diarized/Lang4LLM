#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_digit_char(c: String) -> bool {
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

fn str_to_float(s: String) -> f64 {
    let chs: Vec<String> = s.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut int_part: f64 = 0.0;
    let mut frac_part: f64 = 0.0;
    let mut frac_div: f64 = 1.0;
    let mut in_frac: bool = false;
    let mut i: i64 = 0_i64;
    while i < (chs.len() as i64) {
        if chs[(i) as usize].clone() == ".".to_string() {
            in_frac = true;
        } else {
            if in_frac {
                frac_div = frac_div * 10.0;
                frac_part = frac_part + { let __sf = format!("{}", char_to_digit(chs[(i) as usize].clone())); __sf.parse::<f64>().unwrap_or(0.0) } / frac_div;
            } else {
                int_part = int_part * 10.0 + { let __sf = format!("{}", char_to_digit(chs[(i) as usize].clone())); __sf.parse::<f64>().unwrap_or(0.0) };
            };
        };
        i = i + 1_i64;
    }
    return int_part + frac_part;
}

fn prec(op: String) -> i64 {
    if op == "+".to_string() || op == "-".to_string() {
        return 1_i64;
    };
    if op == "*".to_string() || op == "/".to_string() {
        return 2_i64;
    };
    return 0_i64;
}

fn apply_op(a: f64, b: f64, op: String) -> f64 {
    if op == "+".to_string() {
        return a + b;
    };
    if op == "-".to_string() {
        return a - b;
    };
    if op == "*".to_string() {
        return a * b;
    };
    return a / b;
}

fn eval_expr(expr: String) -> f64 {
    let chs: Vec<String> = expr.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut out_stack: Vec<f64> = vec![];
    let mut op_stack: Vec<String> = vec![];
    let mut i: i64 = 0_i64;
    let mut prev_was_num: bool = false;
    while i < (chs.len() as i64) {
        let c: String = chs[(i) as usize].clone();
        if c == " ".to_string() || c == "\t".to_string() {
            i = i + 1_i64;
        } else {
            if is_digit_char(c.clone()) {
                let mut num_str: String = "".to_string();
                while i < (chs.len() as i64) && (is_digit_char(chs[(i) as usize].clone()) || chs[(i) as usize].clone() == ".".to_string()) {
                    num_str = format!("{}{}", num_str, format!("{}", chs[(i) as usize].clone()));
                    i = i + 1_i64;
                }
                out_stack.push(str_to_float(num_str.clone()));
                prev_was_num = true;
            } else {
                if c == "(".to_string() {
                    op_stack.push("(".to_string());
                    prev_was_num = false;
                    i = i + 1_i64;
                } else {
                    if c == ")".to_string() {
                        while (op_stack.len() as i64) > 0_i64 && op_stack[((op_stack.len() as i64) - 1_i64) as usize].clone() != "(".to_string() {
                            let top_op: String = op_stack.pop().unwrap();
                            let b: f64 = out_stack.pop().unwrap();
                            let a: f64 = out_stack.pop().unwrap();
                            out_stack.push(apply_op(a.clone(), b.clone(), top_op.clone()));
                        }
                        if (op_stack.len() as i64) > 0_i64 {
                            op_stack.pop().unwrap();
                        };
                        prev_was_num = true;
                        i = i + 1_i64;
                    } else {
                        if c == "-".to_string() && !prev_was_num {
                            out_stack.push(0.0);
                        };
                        while (op_stack.len() as i64) > 0_i64 && op_stack[((op_stack.len() as i64) - 1_i64) as usize].clone() != "(".to_string() && prec(op_stack[((op_stack.len() as i64) - 1_i64) as usize].clone()) >= prec(c.clone()) {
                            let top_op2: String = op_stack.pop().unwrap();
                            let b2: f64 = out_stack.pop().unwrap();
                            let a2: f64 = out_stack.pop().unwrap();
                            out_stack.push(apply_op(a2.clone(), b2.clone(), top_op2.clone()));
                        }
                        op_stack.push(c.clone());
                        prev_was_num = false;
                        i = i + 1_i64;
                    };
                };
            };
        };
    }
    while (op_stack.len() as i64) > 0_i64 {
        let top_op3: String = op_stack.pop().unwrap();
        let b3: f64 = out_stack.pop().unwrap();
        let a3: f64 = out_stack.pop().unwrap();
        out_stack.push(apply_op(a3.clone(), b3.clone(), top_op3.clone()));
    }
    return out_stack[(0_i64) as usize].clone();
}

fn main() {
    println!("{}", format!("3 + 5 = {}", format!("{}", eval_expr("3 + 5".to_string()))));
    println!("{}", format!("2 + 3 * 4 = {}", format!("{}", eval_expr("2 + 3 * 4".to_string()))));
    println!("{}", format!("(2 + 3) * 4 = {}", format!("{}", eval_expr("(2 + 3) * 4".to_string()))));
    println!("{}", format!("10 / 2 + 3 = {}", format!("{}", eval_expr("10 / 2 + 3".to_string()))));
    println!("{}", format!("2 * (3 + 4) - 10 / 5 = {}", format!("{}", eval_expr("2 * (3 + 4) - 10 / 5".to_string()))));
    println!("{}", format!("-5 + 3 = {}", format!("{}", eval_expr("-5 + 3".to_string()))));
    println!("{}", format!("10 - 3 - 2 = {}", format!("{}", eval_expr("10 - 3 - 2".to_string()))));
    println!("{}", format!("42 = {}", format!("{}", eval_expr("42".to_string()))));
    println!("{}", format!("3.5 * 2 = {}", format!("{}", eval_expr("3.5 * 2".to_string()))));
}

