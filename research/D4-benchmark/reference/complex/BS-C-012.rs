#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<f64>,
    rows: i64,
    cols: i64,
}

fn mat_get(m: Matrix, r: i64, c: i64) -> f64 {
    return m.data[(r * m.cols + c) as usize].clone();
}

fn mat_new(rows: i64, cols: i64) -> Matrix {
    let mut d: Vec<f64> = vec![];
    let mut i: i64 = 0_i64;
    while i < rows * cols {
        d.push(0.0);
        i = i + 1_i64;
    }
    return Matrix { data: d, rows: rows, cols: cols };
}

fn row_to_string(m: Matrix, r: i64) -> String {
    let mut s: String = "[".to_string();
    let mut j: i64 = 0_i64;
    while j < m.cols {
        if j > 0_i64 {
            s = format!("{}, ", s);
        };
        s = format!("{}{}", s, format!("{}", mat_get(m.clone(), r.clone(), j.clone())));
        j = j + 1_i64;
    }
    s = format!("{}]", s);
    return s;
}

fn mat_to_string(m: Matrix) -> String {
    let mut s: String = "".to_string();
    let mut i: i64 = 0_i64;
    while i < m.rows {
        if i > 0_i64 {
            s = format!("{}; ", s);
        };
        s = format!("{}{}", s, format!("{}", row_to_string(m.clone(), i.clone())));
        i = i + 1_i64;
    }
    return s;
}

fn print_mat(label: String, m: Matrix) {
    let mut i: i64 = 0_i64;
    while i < m.rows {
        println!("{}", format!("{} row {}: {}", label, format!("{}", i), row_to_string(m.clone(), i.clone())));
        i = i + 1_i64;
    }
}

fn multiply(a: Matrix, b: Matrix) -> String {
    if a.cols != b.rows {
        return "error: incompatible dimensions".to_string();
    };
    let mut res: Matrix = mat_new(a.rows.clone(), b.cols.clone());
    let mut i: i64 = 0_i64;
    while i < a.rows {
        let mut j: i64 = 0_i64;
        while j < b.cols {
            let mut sum: f64 = 0.0;
            let mut k: i64 = 0_i64;
            while k < a.cols {
                sum = sum + mat_get(a.clone(), i.clone(), k.clone()) * mat_get(b.clone(), k.clone(), j.clone());
                k = k + 1_i64;
            }
            res.data[(i * res.cols + j) as usize] = sum;
            j = j + 1_i64;
        }
        i = i + 1_i64;
    }
    return mat_to_string(res.clone());
}

fn transpose(a: Matrix) -> Matrix {
    let mut res: Matrix = mat_new(a.cols.clone(), a.rows.clone());
    let mut i: i64 = 0_i64;
    while i < a.rows {
        let mut j: i64 = 0_i64;
        while j < a.cols {
            res.data[(j * res.cols + i) as usize] = mat_get(a.clone(), i.clone(), j.clone());
            j = j + 1_i64;
        }
        i = i + 1_i64;
    }
    return res;
}

fn determinant(a: Matrix) -> String {
    if a.rows != a.cols {
        return "error: non-square matrix".to_string();
    };
    if a.rows == 1_i64 {
        return format!("{}", mat_get(a.clone(), 0_i64, 0_i64));
    };
    if a.rows == 2_i64 {
        let det2: f64 = mat_get(a.clone(), 0_i64, 0_i64) * mat_get(a.clone(), 1_i64, 1_i64) - mat_get(a.clone(), 0_i64, 1_i64) * mat_get(a.clone(), 1_i64, 0_i64);
        return format!("{}", det2);
    };
    if a.rows == 3_i64 {
        let av: f64 = mat_get(a.clone(), 0_i64, 0_i64);
        let bv: f64 = mat_get(a.clone(), 0_i64, 1_i64);
        let cv: f64 = mat_get(a.clone(), 0_i64, 2_i64);
        let dv: f64 = mat_get(a.clone(), 1_i64, 0_i64);
        let ev: f64 = mat_get(a.clone(), 1_i64, 1_i64);
        let fv: f64 = mat_get(a.clone(), 1_i64, 2_i64);
        let gv: f64 = mat_get(a.clone(), 2_i64, 0_i64);
        let hv: f64 = mat_get(a.clone(), 2_i64, 1_i64);
        let iv: f64 = mat_get(a.clone(), 2_i64, 2_i64);
        let det3: f64 = av * (ev * iv - fv * hv) - bv * (dv * iv - fv * gv) + cv * (dv * hv - ev * gv);
        return format!("{}", det3);
    };
    return "error: matrix too large for determinant".to_string();
}

fn main() {
    let mut a2: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0], rows: 2_i64, cols: 2_i64 };
    let mut b2: Matrix = Matrix { data: vec![5.0, 6.0, 7.0, 8.0], rows: 2_i64, cols: 2_i64 };
    println!("{}", "multiply 2x2:".to_string());
    println!("{}", multiply(a2.clone(), b2.clone()));
    let mut a23: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], rows: 2_i64, cols: 3_i64 };
    let mut b32: Matrix = Matrix { data: vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0], rows: 3_i64, cols: 2_i64 };
    println!("{}", "multiply 2x3 * 3x2:".to_string());
    println!("{}", multiply(a23.clone(), b32.clone()));
    let mut am: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0], rows: 2_i64, cols: 2_i64 };
    let mut bm: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], rows: 3_i64, cols: 2_i64 };
    println!("{}", format!("mismatch: {}", multiply(am.clone(), bm.clone())));
    let mut t23: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], rows: 2_i64, cols: 3_i64 };
    let mut tr: Matrix = transpose(t23.clone());
    println!("{}", "transpose 2x3 -> 3x2:".to_string());
    print_mat("T".to_string(), tr.clone());
    let mut d1: Matrix = Matrix { data: vec![7.0], rows: 1_i64, cols: 1_i64 };
    println!("{}", format!("det 1x1: {}", determinant(d1.clone())));
    let mut d2: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0], rows: 2_i64, cols: 2_i64 };
    println!("{}", format!("det 2x2: {}", determinant(d2.clone())));
    let mut d3: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], rows: 3_i64, cols: 3_i64 };
    println!("{}", format!("det 3x3: {}", determinant(d3.clone())));
    let mut dns: Matrix = Matrix { data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], rows: 2_i64, cols: 3_i64 };
    println!("{}", format!("det non-square: {}", determinant(dns.clone())));
    let mut id2: Matrix = Matrix { data: vec![1.0, 0.0, 0.0, 1.0], rows: 2_i64, cols: 2_i64 };
    let mut orig: Matrix = Matrix { data: vec![3.0, 7.0, 2.0, 5.0], rows: 2_i64, cols: 2_i64 };
    println!("{}", "A * I:".to_string());
    println!("{}", multiply(orig.clone(), id2.clone()));
}

