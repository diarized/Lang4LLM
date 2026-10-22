#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn parse_positive_int(s: String) -> Result<i64, String> {
    let value: i64 = { let __s = format!("{}", s); __s.parse::<i64>().unwrap_or(0) };
    if value > 0_i64 {
        return Result::Ok(value);
    } else {
        return Result::Err("Value must be positive".to_string());
    }
}

fn process_input(s: String) -> String {
    let result: Result<i64, String> = parse_positive_int(s);
    let output: String = match result {
        Result::Ok(value) => format!("Valid: {}", value),
        Result::Err(msg) => format!("Error: {}", msg),
    };
    return output;
}

fn main() {
    let output: String = process_input("42".to_string());
    println!("{}", output);
    return ();
}

