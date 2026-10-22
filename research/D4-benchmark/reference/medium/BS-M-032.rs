#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn is_alnum_char(c: String) -> bool {
    return c >= "a".to_string() && c <= "z".to_string() || c >= "A".to_string() && c <= "Z".to_string() || c >= "0".to_string() && c <= "9".to_string();
}

fn strip_punct(word: String) -> String {
    let chs: Vec<String> = word.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let n: i64 = (chs.len() as i64);
    if n == 0_i64 {
        return "".to_string();
    };
    let mut left: i64 = 0_i64;
    let mut right: i64 = n - 1_i64;
    while left <= right && !is_alnum_char(chs[(left) as usize].clone()) {
        left = left + 1_i64;
    }
    while right >= left && !is_alnum_char(chs[(right) as usize].clone()) {
        right = right - 1_i64;
    }
    if left > right {
        return "".to_string();
    };
    let mut result: String = "".to_string();
    for i in left..right + 1_i64 {
        result = result + chs[(i) as usize].clone();
    }
    return result;
}

fn tokenize(text: String) -> Vec<String> {
    let chs: Vec<String> = text.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let n: i64 = (chs.len() as i64);
    let mut tokens: Vec<String> = vec![];
    let mut cur: String = "".to_string();
    for i in 0_i64..n {
        let c: String = chs[(i) as usize].clone();
        if c == " ".to_string() || c == "\t".to_string() || c == "\n".to_string() || c == "\r".to_string() {
            if cur != "".to_string() {
                tokens.push(cur);
                cur = "".to_string();
            };
        } else {
            cur = cur + c;
        };
    }
    if cur != "".to_string() {
        tokens.push(cur);
    };
    return tokens;
}

fn find_word(words: Vec<String>, word: String) -> i64 {
    let mut idx: i64 = -1_i64;
    for i in 0_i64..(words.len() as i64) {
        if words[(i) as usize].clone() == word {
            idx = i;
        };
    }
    return idx;
}

fn word_freq_words(text: String) -> Vec<String> {
    let raw_tokens: Vec<String> = tokenize(text.clone());
    let mut wwords: Vec<String> = vec![];
    let mut wcounts: Vec<i64> = vec![];
    for i in 0_i64..(raw_tokens.len() as i64) {
        let stripped: String = strip_punct(raw_tokens[(i) as usize].clone());
        if stripped != "".to_string() {
            let lower_w: String = stripped.to_lowercase();
            let existing: i64 = find_word(wwords.clone(), lower_w.clone());
            if existing >= 0_i64 {
                wcounts[(existing) as usize] = wcounts[(existing) as usize].clone() + 1_i64;
            } else {
                wwords.push(lower_w);
                wcounts.push(1_i64);
            };
        };
    }
    let n: i64 = (wwords.len() as i64);
    for i in 1_i64..n {
        let key_w: String = wwords[(i) as usize].clone();
        let key_c: i64 = wcounts[(i) as usize].clone();
        let mut j: i64 = i - 1_i64;
        let mut moved: bool = false;
        while j >= 0_i64 && !moved {
            let cmp_c: i64 = wcounts[(j) as usize].clone();
            let cmp_w: String = wwords[(j) as usize].clone();
            let should_swap: bool = cmp_c < key_c || cmp_c == key_c && cmp_w > key_w;
            if should_swap {
                wwords[(j + 1_i64) as usize] = cmp_w;
                wcounts[(j + 1_i64) as usize] = cmp_c;
                j = j - 1_i64;
            } else {
                moved = true;
            };
        }
        wwords[(j + 1_i64) as usize] = key_w;
        wcounts[(j + 1_i64) as usize] = key_c;
    }
    return wwords;
}

fn word_freq_counts(text: String) -> Vec<i64> {
    let raw_tokens: Vec<String> = tokenize(text.clone());
    let mut wwords: Vec<String> = vec![];
    let mut wcounts: Vec<i64> = vec![];
    for i in 0_i64..(raw_tokens.len() as i64) {
        let stripped: String = strip_punct(raw_tokens[(i) as usize].clone());
        if stripped != "".to_string() {
            let lower_w: String = stripped.to_lowercase();
            let existing: i64 = find_word(wwords.clone(), lower_w.clone());
            if existing >= 0_i64 {
                wcounts[(existing) as usize] = wcounts[(existing) as usize].clone() + 1_i64;
            } else {
                wwords.push(lower_w);
                wcounts.push(1_i64);
            };
        };
    }
    let n: i64 = (wwords.len() as i64);
    for i in 1_i64..n {
        let key_w: String = wwords[(i) as usize].clone();
        let key_c: i64 = wcounts[(i) as usize].clone();
        let mut j: i64 = i - 1_i64;
        let mut moved: bool = false;
        while j >= 0_i64 && !moved {
            let cmp_c: i64 = wcounts[(j) as usize].clone();
            let cmp_w: String = wwords[(j) as usize].clone();
            let should_swap: bool = cmp_c < key_c || cmp_c == key_c && cmp_w > key_w;
            if should_swap {
                wwords[(j + 1_i64) as usize] = cmp_w;
                wcounts[(j + 1_i64) as usize] = cmp_c;
                j = j - 1_i64;
            } else {
                moved = true;
            };
        }
        wwords[(j + 1_i64) as usize] = key_w;
        wcounts[(j + 1_i64) as usize] = key_c;
    }
    return wcounts;
}

fn print_word_freq(text: String) {
    let ww: Vec<String> = word_freq_words(text.clone());
    let wc: Vec<i64> = word_freq_counts(text.clone());
    if ww.is_empty() {
        println!("{}", "[]".to_string());
        return;
    };
    let mut s: String = "[".to_string();
    for i in 0_i64..(ww.len() as i64) {
        if i > 0_i64 {
            s = format!("{}, ", s);
        };
        s = format!("{}({}, {})", s, ww[(i) as usize].clone(), format!("{}", wc[(i) as usize].clone()));
    }
    s = format!("{}]", s);
    println!("{}", s);
}

fn main() {
    print_word_freq("the cat sat on the mat".to_string());
    print_word_freq("Hello hello HELLO".to_string());
    print_word_freq("Hello, world! Hello.".to_string());
    print_word_freq("To be, or not to be,\nthat is the question.".to_string());
    print_word_freq("".to_string());
    print_word_freq("wow!!! yes... no?".to_string());
    print_word_freq("item 1 item 2 item 1".to_string());
}

