#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn repeat_str(s: String, n: i64) -> String {
    let mut result: String = "".to_string();
    let mut i: i64 = 0_i64;
    while i < n {
        result = format!("{}{}", result, format!("{}", s));
        i = i + 1_i64;
    }
    return result;
}

fn pad_right(s: String, width: i64) -> String {
    let pad: i64 = width - (s.len() as i64);
    if pad <= 0_i64 {
        return s;
    };
    return format!("{}{}", s, format!("{}", repeat_str(" ".to_string(), pad.clone())));
}

fn pad_left(s: String, width: i64) -> String {
    let pad: i64 = width - (s.len() as i64);
    if pad <= 0_i64 {
        return s;
    };
    return format!("{}{}", repeat_str(" ".to_string(), pad.clone()), format!("{}", s));
}

fn max_int(a: i64, b: i64) -> i64 {
    if a > b {
        return a;
    };
    return b;
}

fn format_score(v: f64) -> String {
    return format!("{}", v);
}

fn generate_report(title: String, names: Vec<String>, s1: Vec<f64>, s2: Vec<f64>, s3: Vec<f64>, ncols: i64) {
    let n: i64 = (names.len() as i64);
    let mut tot1: f64 = 0.0;
    let mut tot2: f64 = 0.0;
    let mut tot3: f64 = 0.0;
    let mut i: i64 = 0_i64;
    while i < n {
        tot1 = tot1 + s1[(i) as usize].clone();
        if ncols >= 2_i64 {
            tot2 = tot2 + s2[(i) as usize].clone();
        };
        if ncols >= 3_i64 {
            tot3 = tot3 + s3[(i) as usize].clone();
        };
        i = i + 1_i64;
    }
    let mut name_w: i64 = ("Name".to_string().len() as i64);
    let mut j: i64 = 0_i64;
    while j < n {
        name_w = max_int(name_w.clone(), (names[(j) as usize].clone().len() as i64));
        j = j + 1_i64;
    }
    name_w = max_int(name_w.clone(), ("TOTAL".to_string().len() as i64));
    let mut w1: i64 = ("Score 1".to_string().len() as i64);
    let mut k: i64 = 0_i64;
    while k < n {
        w1 = max_int(w1.clone(), (format_score(s1[(k) as usize].clone()).len() as i64));
        k = k + 1_i64;
    }
    w1 = max_int(w1.clone(), (format_score(tot1.clone()).len() as i64));
    let mut w2: i64 = ("Score 2".to_string().len() as i64);
    if ncols >= 2_i64 {
        let mut k2: i64 = 0_i64;
        while k2 < n {
            w2 = max_int(w2.clone(), (format_score(s2[(k2) as usize].clone()).len() as i64));
            k2 = k2 + 1_i64;
        }
        w2 = max_int(w2.clone(), (format_score(tot2.clone()).len() as i64));
    };
    let mut w3: i64 = ("Score 3".to_string().len() as i64);
    if ncols >= 3_i64 {
        let mut k3: i64 = 0_i64;
        while k3 < n {
            w3 = max_int(w3.clone(), (format_score(s3[(k3) as usize].clone()).len() as i64));
            k3 = k3 + 1_i64;
        }
        w3 = max_int(w3.clone(), (format_score(tot3.clone()).len() as i64));
    };
    let mut total_w: i64 = name_w + 3_i64 + w1;
    if ncols >= 2_i64 {
        total_w = total_w + 3_i64 + w2;
    };
    if ncols >= 3_i64 {
        total_w = total_w + 3_i64 + w3;
    };
    let sep: String = repeat_str("-".to_string(), total_w.clone());
    println!("{}", title);
    println!("{}", sep);
    let mut header: String = format!("{} | {}", pad_right("Name".to_string(), name_w.clone()), pad_left("Score 1".to_string(), w1.clone()));
    if ncols >= 2_i64 {
        header = format!("{} | {}", header, pad_left("Score 2".to_string(), w2.clone()));
    };
    if ncols >= 3_i64 {
        header = format!("{} | {}", header, pad_left("Score 3".to_string(), w3.clone()));
    };
    println!("{}", header);
    println!("{}", sep);
    let mut r: i64 = 0_i64;
    while r < n {
        let mut row: String = format!("{} | {}", pad_right(names[(r) as usize].clone(), name_w.clone()), pad_left(format_score(s1[(r) as usize].clone()), w1.clone()));
        if ncols >= 2_i64 {
            row = format!("{} | {}", row, pad_left(format_score(s2[(r) as usize].clone()), w2.clone()));
        };
        if ncols >= 3_i64 {
            row = format!("{} | {}", row, pad_left(format_score(s3[(r) as usize].clone()), w3.clone()));
        };
        println!("{}", row);
        r = r + 1_i64;
    }
    println!("{}", sep);
    let mut trow: String = format!("{} | {}", pad_right("TOTAL".to_string(), name_w.clone()), pad_left(format_score(tot1.clone()), w1.clone()));
    if ncols >= 2_i64 {
        trow = format!("{} | {}", trow, pad_left(format_score(tot2.clone()), w2.clone()));
    };
    if ncols >= 3_i64 {
        trow = format!("{} | {}", trow, pad_left(format_score(tot3.clone()), w3.clone()));
    };
    println!("{}", trow);
    println!("{}", sep);
}

fn main() {
    let names1: Vec<String> = vec!["Alice".to_string(), "Bob".to_string()];
    let sc1a: Vec<f64> = vec![85.0, 78.0];
    let sc1b: Vec<f64> = vec![92.0, 88.0];
    let empty1: Vec<f64> = vec![0.0, 0.0];
    generate_report("Student Scores".to_string(), names1.clone(), sc1a.clone(), sc1b.clone(), empty1.clone(), 2_i64);
    let names2: Vec<String> = vec!["Team A".to_string(), "Team B".to_string(), "Team C".to_string()];
    let sc2a: Vec<f64> = vec![42.0, 38.0, 55.0];
    let empty2a: Vec<f64> = vec![0.0, 0.0, 0.0];
    generate_report("Q1 Results".to_string(), names2.clone(), sc2a.clone(), empty2a.clone(), empty2a.clone(), 1_i64);
    let names3: Vec<String> = vec!["Alice".to_string(), "Bob".to_string(), "Christopher".to_string()];
    let sc3a: Vec<f64> = vec![10.0, 20.0, 30.0];
    let sc3b: Vec<f64> = vec![5.0, 15.0, 25.0];
    let empty3: Vec<f64> = vec![0.0, 0.0, 0.0];
    generate_report("Name Width Test".to_string(), names3.clone(), sc3a.clone(), sc3b.clone(), empty3.clone(), 2_i64);
}

