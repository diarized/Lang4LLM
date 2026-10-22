#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn config_get(config_text: String, key: String) -> String {
    let lines: Vec<String> = config_text.split("\n".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    let mut result: String = "".to_string();
    for i in 0_i64..(lines.len() as i64) {
        let line: String = lines[(i) as usize].clone().trim().to_string();
        if line.is_empty() {
        } else {
            if line.starts_with("#".to_string().as_str()) {
            } else {
                let line_chars: Vec<String> = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
                let mut eq_pos: i64 = -1_i64;
                let mut found_eq: bool = false;
                for j in 0_i64..(line_chars.len() as i64) {
                    if line_chars[(j) as usize].clone() == "=".to_string() && !found_eq {
                        eq_pos = j;
                        found_eq = true;
                    };
                }
                if found_eq {
                    let mut k: String = "".to_string();
                    for j in 0_i64..eq_pos {
                        k = k + line_chars[(j) as usize].clone();
                    }
                    let mut v: String = "".to_string();
                    for j in eq_pos + 1_i64..(line_chars.len() as i64) {
                        v = v + line_chars[(j) as usize].clone();
                    }
                    let trimmed_k: String = k.trim().to_string();
                    let trimmed_v: String = v.trim().to_string();
                    if trimmed_k == key {
                        result = trimmed_v;
                    };
                };
            };
        };
    }
    return result;
}

fn main() {
    let cfg1: String = "host=localhost\nport=8080\ndebug=true".to_string();
    println!("{}", format!("host = {}", config_get(cfg1.clone(), "host".to_string())));
    println!("{}", format!("port = {}", config_get(cfg1.clone(), "port".to_string())));
    println!("{}", format!("debug = {}", config_get(cfg1.clone(), "debug".to_string())));
    let cfg2: String = "# Database\nhost=db.example.com\n\n# Creds\nuser=admin\npassword=secret".to_string();
    println!("{}", format!("db_host = {}", config_get(cfg2.clone(), "host".to_string())));
    println!("{}", format!("user = {}", config_get(cfg2.clone(), "user".to_string())));
    let cfg3: String = "key1 = value1\nkey2=value2\n  key3  =  value3".to_string();
    println!("{}", format!("key1 = {}", config_get(cfg3.clone(), "key1".to_string())));
    println!("{}", format!("key3 = {}", config_get(cfg3.clone(), "key3".to_string())));
    let cfg4: String = "url=http://example.com/path?a=1&b=2\nformula=x=y+z".to_string();
    println!("{}", format!("url = {}", config_get(cfg4.clone(), "url".to_string())));
    println!("{}", format!("formula = {}", config_get(cfg4.clone(), "formula".to_string())));
    let cfg5: String = "timeout=30\ntimeout=60".to_string();
    println!("{}", format!("timeout = {}", config_get(cfg5.clone(), "timeout".to_string())));
    let cfg6: String = "empty_key=\nanother=value".to_string();
    println!("{}", format!("empty_key = '{}'", config_get(cfg6.clone(), "empty_key".to_string())));
    let cfg7: String = "name=Alice\nage=30".to_string();
    println!("{}", format!("missing = '{}'", config_get(cfg7.clone(), "missing".to_string())));
    let cfg8: String = "valid=yes\nno_equals_sign_here\nanother=ok".to_string();
    println!("{}", format!("valid = {}", config_get(cfg8.clone(), "valid".to_string())));
    println!("{}", format!("another = {}", config_get(cfg8.clone(), "another".to_string())));
}

