#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn char_idx(c: String) -> i64 {
    if c == "a".to_string() {
        return 0_i64;
    };
    if c == "b".to_string() {
        return 1_i64;
    };
    if c == "c".to_string() {
        return 2_i64;
    };
    if c == "d".to_string() {
        return 3_i64;
    };
    if c == "e".to_string() {
        return 4_i64;
    };
    if c == "f".to_string() {
        return 5_i64;
    };
    if c == "g".to_string() {
        return 6_i64;
    };
    if c == "h".to_string() {
        return 7_i64;
    };
    if c == "i".to_string() {
        return 8_i64;
    };
    if c == "j".to_string() {
        return 9_i64;
    };
    if c == "k".to_string() {
        return 10_i64;
    };
    if c == "l".to_string() {
        return 11_i64;
    };
    if c == "m".to_string() {
        return 12_i64;
    };
    if c == "n".to_string() {
        return 13_i64;
    };
    if c == "o".to_string() {
        return 14_i64;
    };
    if c == "p".to_string() {
        return 15_i64;
    };
    if c == "q".to_string() {
        return 16_i64;
    };
    if c == "r".to_string() {
        return 17_i64;
    };
    if c == "s".to_string() {
        return 18_i64;
    };
    if c == "t".to_string() {
        return 19_i64;
    };
    if c == "u".to_string() {
        return 20_i64;
    };
    if c == "v".to_string() {
        return 21_i64;
    };
    if c == "w".to_string() {
        return 22_i64;
    };
    if c == "x".to_string() {
        return 23_i64;
    };
    if c == "y".to_string() {
        return 24_i64;
    };
    return 25_i64;
}

fn idx_to_char(i: i64) -> String {
    if i == 0_i64 {
        return "a".to_string();
    };
    if i == 1_i64 {
        return "b".to_string();
    };
    if i == 2_i64 {
        return "c".to_string();
    };
    if i == 3_i64 {
        return "d".to_string();
    };
    if i == 4_i64 {
        return "e".to_string();
    };
    if i == 5_i64 {
        return "f".to_string();
    };
    if i == 6_i64 {
        return "g".to_string();
    };
    if i == 7_i64 {
        return "h".to_string();
    };
    if i == 8_i64 {
        return "i".to_string();
    };
    if i == 9_i64 {
        return "j".to_string();
    };
    if i == 10_i64 {
        return "k".to_string();
    };
    if i == 11_i64 {
        return "l".to_string();
    };
    if i == 12_i64 {
        return "m".to_string();
    };
    if i == 13_i64 {
        return "n".to_string();
    };
    if i == 14_i64 {
        return "o".to_string();
    };
    if i == 15_i64 {
        return "p".to_string();
    };
    if i == 16_i64 {
        return "q".to_string();
    };
    if i == 17_i64 {
        return "r".to_string();
    };
    if i == 18_i64 {
        return "s".to_string();
    };
    if i == 19_i64 {
        return "t".to_string();
    };
    if i == 20_i64 {
        return "u".to_string();
    };
    if i == 21_i64 {
        return "v".to_string();
    };
    if i == 22_i64 {
        return "w".to_string();
    };
    if i == 23_i64 {
        return "x".to_string();
    };
    if i == 24_i64 {
        return "y".to_string();
    };
    return "z".to_string();
}

#[derive(Debug, Clone)]
struct Trie {
    children: Vec<i64>,
    is_end: Vec<bool>,
    size: i64,
}

fn trie_new() -> Trie {
    let mut ch: Vec<i64> = vec![];
    let mut ie: Vec<bool> = vec![];
    let mut i: i64 = 0_i64;
    while i < 26_i64 {
        ch.push(-1_i64);
        i = i + 1_i64;
    }
    ie.push(false);
    return Trie { children: ch, is_end: ie, size: 1_i64 };
}

fn trie_insert(t: Trie, word: String) -> Trie {
    let chs: Vec<String> = word.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut new_ch: Vec<i64> = t.children.clone();
    let mut new_ie: Vec<bool> = t.is_end.clone();
    let mut new_size: i64 = t.size.clone();
    let mut cur: i64 = 0_i64;
    let mut i: i64 = 0_i64;
    while i < (chs.len() as i64) {
        let ci: i64 = char_idx(chs[(i) as usize].clone());
        let idx: i64 = cur * 26_i64 + ci;
        if new_ch[(idx) as usize].clone() == -1_i64 {
            let node_id: i64 = new_size.clone();
            new_size = new_size + 1_i64;
            new_ch[(idx) as usize] = node_id;
            let mut j: i64 = 0_i64;
            while j < 26_i64 {
                new_ch.push(-1_i64);
                j = j + 1_i64;
            }
            new_ie.push(false);
        };
        cur = new_ch[(idx) as usize].clone();
        i = i + 1_i64;
    }
    new_ie[(cur) as usize] = true;
    return Trie { children: new_ch, is_end: new_ie, size: new_size };
}

fn trie_find_node(t: Trie, prefix: String) -> i64 {
    let chs: Vec<String> = prefix.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut cur: i64 = 0_i64;
    let mut i: i64 = 0_i64;
    while i < (chs.len() as i64) {
        let ci: i64 = char_idx(chs[(i) as usize].clone());
        let idx: i64 = cur * 26_i64 + ci;
        if t.children[(idx) as usize].clone() == -1_i64 {
            return -1_i64;
        };
        cur = t.children[(idx) as usize].clone();
        i = i + 1_i64;
    }
    return cur;
}

fn trie_search(t: Trie, word: String) -> bool {
    let node: i64 = trie_find_node(t.clone(), word.clone());
    if node == -1_i64 {
        return false;
    };
    return t.is_end[(node) as usize].clone();
}

fn trie_starts_with(t: Trie, prefix: String) -> bool {
    if (prefix.len() as i64) == 0_i64 {
        return true;
    };
    let node: i64 = trie_find_node(t.clone(), prefix.clone());
    return node != -1_i64;
}

fn collect_words(ch: Vec<i64>, ie: Vec<bool>, node: i64, prefix: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    if ie[(node) as usize].clone() {
        result.push(prefix.clone());
    };
    let mut i: i64 = 0_i64;
    while i < 26_i64 {
        let child: i64 = ch[(node * 26_i64 + i) as usize].clone();
        if child != -1_i64 {
            let sub: Vec<String> = collect_words(ch.clone(), ie.clone(), child.clone(), format!("{}{}", prefix, format!("{}", idx_to_char(i.clone()))));
            result = { let mut __v = result; __v.extend(sub); __v };
        };
        i = i + 1_i64;
    }
    return result;
}

fn trie_words_with_prefix(t: Trie, prefix: String) -> Vec<String> {
    if (prefix.len() as i64) == 0_i64 {
        return collect_words(t.children.clone(), t.is_end.clone(), 0_i64, "".to_string());
    } else {
        let start_node: i64 = trie_find_node(t.clone(), prefix.clone());
        if start_node != -1_i64 {
            return collect_words(t.children.clone(), t.is_end.clone(), start_node.clone(), prefix.clone());
        } else {
            return vec![];
        };
    };
}

fn main() {
    let t0: Trie = trie_new();
    let t1: Trie = trie_insert(t0.clone(), "apple".to_string());
    let t2: Trie = trie_insert(t1.clone(), "app".to_string());
    let t3: Trie = trie_insert(t2.clone(), "application".to_string());
    println!("{}", format!("search(app) = {}", format!("{}", trie_search(t3.clone(), "app".to_string()))));
    println!("{}", format!("search(ap) = {}", format!("{}", trie_search(t3.clone(), "ap".to_string()))));
    println!("{}", format!("search(apple) = {}", format!("{}", trie_search(t3.clone(), "apple".to_string()))));
    println!("{}", format!("search(apricot) = {}", format!("{}", trie_search(t3.clone(), "apricot".to_string()))));
    let u0: Trie = trie_new();
    let u1: Trie = trie_insert(u0.clone(), "car".to_string());
    let u2: Trie = trie_insert(u1.clone(), "card".to_string());
    let u3: Trie = trie_insert(u2.clone(), "care".to_string());
    let u4: Trie = trie_insert(u3.clone(), "careful".to_string());
    let u5: Trie = trie_insert(u4.clone(), "bat".to_string());
    let mut w1: Vec<String> = trie_words_with_prefix(u5.clone(), "car".to_string());
    w1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{}", format!("words(car) = {}", w1.join(", ".to_string().as_str())));
    let mut w2: Vec<String> = trie_words_with_prefix(u5.clone(), "care".to_string());
    w2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{}", format!("words(care) = {}", w2.join(", ".to_string().as_str())));
    let w3: Vec<String> = trie_words_with_prefix(u5.clone(), "cat".to_string());
    println!("{}", format!("words(cat) count = {}", format!("{}", (w3.len() as i64))));
    println!("{}", format!("starts_with(car) = {}", format!("{}", trie_starts_with(u5.clone(), "car".to_string()))));
    println!("{}", format!("starts_with(xyz) = {}", format!("{}", trie_starts_with(u5.clone(), "xyz".to_string()))));
    let v0: Trie = trie_new();
    let v1: Trie = trie_insert(v0.clone(), "test".to_string());
    let v2: Trie = trie_insert(v1.clone(), "test".to_string());
    let w4: Vec<String> = trie_words_with_prefix(v2.clone(), "test".to_string());
    println!("{}", format!("dup test count = {}", format!("{}", (w4.len() as i64))));
}

