#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct BST {
    val: Vec<i64>,
    left: Vec<i64>,
    right: Vec<i64>,
    root: i64,
    size: i64,
}

fn bst_new() -> BST {
    let v: Vec<i64> = vec![];
    let l: Vec<i64> = vec![];
    let r: Vec<i64> = vec![];
    return BST { val: v, left: l, right: r, root: -1_i64, size: 0_i64 };
}

fn bst_insert(t: BST, value: i64) -> BST {
    let mut new_val: Vec<i64> = t.val.clone();
    let mut new_left: Vec<i64> = t.left.clone();
    let mut new_right: Vec<i64> = t.right.clone();
    if t.root == -1_i64 {
        new_val.push(value.clone());
        new_left.push(-1_i64);
        new_right.push(-1_i64);
        return BST { val: new_val, left: new_left, right: new_right, root: 0_i64, size: 1_i64 };
    };
    let mut cur: i64 = t.root.clone();
    let mut done: bool = false;
    while !done {
        if value < new_val[(cur) as usize].clone() {
            if new_left[(cur) as usize].clone() == -1_i64 {
                let ni: i64 = (new_val.len() as i64);
                new_val.push(value.clone());
                new_left.push(-1_i64);
                new_right.push(-1_i64);
                new_left[(cur) as usize] = ni;
                done = true;
            } else {
                cur = new_left[(cur) as usize].clone();
            };
        } else {
            if value > new_val[(cur) as usize].clone() {
                if new_right[(cur) as usize].clone() == -1_i64 {
                    let ni2: i64 = (new_val.len() as i64);
                    new_val.push(value.clone());
                    new_left.push(-1_i64);
                    new_right.push(-1_i64);
                    new_right[(cur) as usize] = ni2;
                    done = true;
                } else {
                    cur = new_right[(cur) as usize].clone();
                };
            } else {
                done = true;
            };
        };
    }
    let new_size: i64 = (new_val.len() as i64);
    return BST { val: new_val, left: new_left, right: new_right, root: t.root, size: new_size };
}

fn bst_search(t: BST, value: i64) -> bool {
    let mut cur: i64 = t.root.clone();
    while cur != -1_i64 {
        if t.val[(cur) as usize].clone() == value {
            return true;
        };
        if value < t.val[(cur) as usize].clone() {
            cur = t.left[(cur) as usize].clone();
        } else {
            cur = t.right[(cur) as usize].clone();
        };
    }
    return false;
}

fn bst_inorder_rec(val: Vec<i64>, left: Vec<i64>, right: Vec<i64>, node: i64) -> Vec<i64> {
    if node == -1_i64 {
        return vec![];
    };
    let left_res: Vec<i64> = bst_inorder_rec(val.clone(), left.clone(), right.clone(), left[(node) as usize].clone());
    let right_res: Vec<i64> = bst_inorder_rec(val.clone(), left.clone(), right.clone(), right[(node) as usize].clone());
    let mut mid: Vec<i64> = vec![];
    mid.push(val[(node) as usize].clone());
    let lr: Vec<i64> = { let mut __v = left_res; __v.extend(mid); __v };
    return { let mut __v = lr; __v.extend(right_res); __v };
}

fn bst_inorder(t: BST) -> Vec<i64> {
    return bst_inorder_rec(t.val.clone(), t.left.clone(), t.right.clone(), t.root.clone());
}

fn build_bst_from_sorted(sorted: Vec<i64>) -> BST {
    let mut t: BST = bst_new();
    let mut i: i64 = 0_i64;
    while i < (sorted.len() as i64) {
        t = bst_insert(t.clone(), sorted[(i) as usize].clone());
        i = i + 1_i64;
    }
    return t;
}

fn bst_delete(t: BST, value: i64) -> BST {
    let inorder: Vec<i64> = bst_inorder(t.clone());
    let mut filtered: Vec<i64> = vec![];
    let mut i: i64 = 0_i64;
    let mut found: bool = false;
    while i < (inorder.len() as i64) {
        if inorder[(i) as usize].clone() == value && !found {
            found = true;
        } else {
            filtered.push(inorder[(i) as usize].clone());
        };
        i = i + 1_i64;
    }
    return build_bst_from_sorted(filtered.clone());
}

fn arr_str(arr: Vec<i64>) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    };
    let mut s: String = "[".to_string();
    let mut i: i64 = 0_i64;
    while i < (arr.len() as i64) {
        if i > 0_i64 {
            s = format!("{}, ", s);
        };
        s = format!("{}{}", s, format!("{}", arr[(i) as usize].clone()));
        i = i + 1_i64;
    }
    return format!("{}]", s);
}

fn main() {
    let t0: BST = bst_new();
    let t1: BST = bst_insert(t0.clone(), 5_i64);
    let t2: BST = bst_insert(t1.clone(), 3_i64);
    let t3: BST = bst_insert(t2.clone(), 7_i64);
    let t4: BST = bst_insert(t3.clone(), 1_i64);
    let t5: BST = bst_insert(t4.clone(), 4_i64);
    println!("{}", format!("inorder = {}", arr_str(bst_inorder(t5.clone()))));
    println!("{}", format!("search(3) = {}", format!("{}", bst_search(t5.clone(), 3_i64))));
    println!("{}", format!("search(6) = {}", format!("{}", bst_search(t5.clone(), 6_i64))));
    let t6: BST = bst_insert(t5.clone(), 5_i64);
    println!("{}", format!("dup insert(5) = {}", arr_str(bst_inorder(t6.clone()))));
    let u0: BST = bst_new();
    let u1: BST = bst_insert(u0.clone(), 8_i64);
    let u2: BST = bst_insert(u1.clone(), 4_i64);
    let u3: BST = bst_insert(u2.clone(), 12_i64);
    let u4: BST = bst_insert(u3.clone(), 2_i64);
    let u5: BST = bst_insert(u4.clone(), 6_i64);
    let u6: BST = bst_insert(u5.clone(), 10_i64);
    let u7: BST = bst_insert(u6.clone(), 14_i64);
    let u8: BST = bst_delete(u7.clone(), 2_i64);
    println!("{}", format!("delete(2) = {}", arr_str(bst_inorder(u8.clone()))));
    let u9: BST = bst_insert(u8.clone(), 5_i64);
    let u10: BST = bst_delete(u9.clone(), 6_i64);
    println!("{}", format!("delete(6) = {}", arr_str(bst_inorder(u10.clone()))));
    let u11: BST = bst_delete(u10.clone(), 12_i64);
    println!("{}", format!("delete(12) = {}", arr_str(bst_inorder(u11.clone()))));
    let u12: BST = bst_delete(u11.clone(), 99_i64);
    println!("{}", format!("delete(99) = {}", arr_str(bst_inorder(u12.clone()))));
    let v0: BST = bst_new();
    println!("{}", format!("empty inorder = {}", arr_str(bst_inorder(v0.clone()))));
    println!("{}", format!("search empty = {}", format!("{}", bst_search(v0.clone(), 5_i64))));
}

