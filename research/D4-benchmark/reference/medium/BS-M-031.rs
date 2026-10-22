#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn csv_parse_headers(csv: String) -> Vec<String> {
    let all_chars: Vec<String> = csv.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let total: i64 = (all_chars.len() as i64);
    let mut headers: Vec<String> = vec![];
    let mut field: String = "".to_string();
    let mut in_q: bool = false;
    let mut done: bool = false;
    let mut i: i64 = 0_i64;
    while i < total && !done {
        let ch: String = all_chars[(i) as usize].clone();
        if in_q {
            if ch == "\"".to_string() {
                if i + 1_i64 < total && all_chars[(i + 1_i64) as usize].clone() == "\"".to_string() {
                    field = format!("{}\"", field);
                    i = i + 1_i64;
                } else {
                    in_q = false;
                };
            } else {
                field = format!("{}{}", field, format!("{}", ch));
            };
        } else {
            if ch == "\"".to_string() {
                in_q = true;
            } else {
                if ch == ",".to_string() {
                    headers.push(field);
                    field = "".to_string();
                } else {
                    if ch == "\n".to_string() {
                        headers.push(field);
                        done = true;
                    } else {
                        field = format!("{}{}", field, format!("{}", ch));
                    };
                };
            };
        };
        i = i + 1_i64;
    }
    if !done {
        headers.push(field);
    };
    return headers;
}

fn csv_parse_row(csv: String, row_idx: i64) -> Vec<String> {
    let all_chars: Vec<String> = csv.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let total: i64 = (all_chars.len() as i64);
    let mut newlines_needed: i64 = row_idx + 1_i64;
    let mut i: i64 = 0_i64;
    let mut in_q_skip: bool = false;
    while i < total && newlines_needed > 0_i64 {
        let ch_s: String = all_chars[(i) as usize].clone();
        if in_q_skip {
            if ch_s == "\"".to_string() {
                if i + 1_i64 < total && all_chars[(i + 1_i64) as usize].clone() == "\"".to_string() {
                    i = i + 1_i64;
                } else {
                    in_q_skip = false;
                };
            };
        } else {
            if ch_s == "\"".to_string() {
                in_q_skip = true;
            } else {
                if ch_s == "\n".to_string() {
                    newlines_needed = newlines_needed - 1_i64;
                };
            };
        };
        i = i + 1_i64;
    }
    let mut fields: Vec<String> = vec![];
    let mut field: String = "".to_string();
    let mut in_q2: bool = false;
    let mut done2: bool = false;
    while i < total && !done2 {
        let ch2: String = all_chars[(i) as usize].clone();
        if in_q2 {
            if ch2 == "\"".to_string() {
                if i + 1_i64 < total && all_chars[(i + 1_i64) as usize].clone() == "\"".to_string() {
                    field = format!("{}\"", field);
                    i = i + 1_i64;
                } else {
                    in_q2 = false;
                };
            } else {
                field = format!("{}{}", field, format!("{}", ch2));
            };
        } else {
            if ch2 == "\"".to_string() {
                in_q2 = true;
            } else {
                if ch2 == ",".to_string() {
                    fields.push(field);
                    field = "".to_string();
                } else {
                    if ch2 == "\n".to_string() {
                        fields.push(field);
                        done2 = true;
                    } else {
                        field = format!("{}{}", field, format!("{}", ch2));
                    };
                };
            };
        };
        i = i + 1_i64;
    }
    if !done2 {
        fields.push(field);
    };
    return fields;
}

fn arr_str_to_str(arr: Vec<String>) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    };
    let mut s: String = "[".to_string();
    for i in 0_i64..(arr.len() as i64) {
        if i > 0_i64 {
            s = format!("{}, ", s);
        };
        s = format!("{}\"{}\"", s, arr[(i) as usize].clone());
    }
    s = format!("{}]", s);
    return s;
}

fn main() {
    let csv1: String = "name,age\nAlice,30\nBob,25".to_string();
    println!("{}", format!("headers1 = {}", arr_str_to_str(csv_parse_headers(csv1.clone()))));
    println!("{}", format!("row1[0] = {}", arr_str_to_str(csv_parse_row(csv1.clone(), 0_i64))));
    println!("{}", format!("row1[1] = {}", arr_str_to_str(csv_parse_row(csv1.clone(), 1_i64))));
    let csv2: String = "name,address\nAlice,\"123 Main St, Apt 4\"\nBob,456 Oak Ave".to_string();
    println!("{}", format!("headers2 = {}", arr_str_to_str(csv_parse_headers(csv2.clone()))));
    println!("{}", format!("row2[0] = {}", arr_str_to_str(csv_parse_row(csv2.clone(), 0_i64))));
    println!("{}", format!("row2[1] = {}", arr_str_to_str(csv_parse_row(csv2.clone(), 1_i64))));
    let csv3: String = "id,description\n1,\"He said \"\"hello\"\"\"".to_string();
    println!("{}", format!("headers3 = {}", arr_str_to_str(csv_parse_headers(csv3.clone()))));
    println!("{}", format!("row3[0] = {}", arr_str_to_str(csv_parse_row(csv3.clone(), 0_i64))));
    let csv4: String = "a,b,c\n1,,3".to_string();
    println!("{}", format!("headers4 = {}", arr_str_to_str(csv_parse_headers(csv4.clone()))));
    println!("{}", format!("row4[0] = {}", arr_str_to_str(csv_parse_row(csv4.clone(), 0_i64))));
    let csv5: String = "first,last,score\nAlice,Smith,95\nBob,Jones,82\nCarol,White,78".to_string();
    println!("{}", format!("headers5 = {}", arr_str_to_str(csv_parse_headers(csv5.clone()))));
    println!("{}", format!("row5[0] = {}", arr_str_to_str(csv_parse_row(csv5.clone(), 0_i64))));
    println!("{}", format!("row5[1] = {}", arr_str_to_str(csv_parse_row(csv5.clone(), 1_i64))));
    println!("{}", format!("row5[2] = {}", arr_str_to_str(csv_parse_row(csv5.clone(), 2_i64))));
}

