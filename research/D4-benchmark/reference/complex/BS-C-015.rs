#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Config {
    keys: Vec<String>,
    values: Vec<String>,
}

fn str_array_contains(arr: Vec<String>, val: String) -> bool {
    let mut ci: i64 = 0_i64;
    while ci < (arr.len() as i64) {
        if arr[(ci) as usize].clone() == val {
            return true;
        };
        ci = ci + 1_i64;
    }
    return false;
}

fn config_new() -> Config {
    let empty_keys: Vec<String> = vec![];
    let empty_vals: Vec<String> = vec![];
    return Config { keys: empty_keys, values: empty_vals };
}

fn config_find(cfg: Config, path: String) -> i64 {
    let mut i: i64 = 0_i64;
    while i < (cfg.keys.len() as i64) {
        if cfg.keys[(i) as usize].clone() == path {
            return i;
        };
        i = i + 1_i64;
    }
    return -1_i64;
}

fn config_get(cfg: Config, path: String) -> String {
    let idx: i64 = config_find(cfg.clone(), path.clone());
    if idx == -1_i64 {
        return "null".to_string();
    };
    return cfg.values[(idx) as usize].clone();
}

fn config_set(cfg: Config, path: String, value: String) -> Config {
    let idx: i64 = config_find(cfg.clone(), path.clone());
    if idx == -1_i64 {
        let mut new_keys: Vec<String> = cfg.keys.clone();
        let mut new_vals: Vec<String> = cfg.values.clone();
        new_keys.push(path.clone());
        new_vals.push(value.clone());
        return Config { keys: new_keys, values: new_vals };
    } else {
        let mut updated_vals: Vec<String> = cfg.values.clone();
        updated_vals[(idx) as usize] = value;
        return Config { keys: cfg.keys, values: updated_vals };
    };
}

fn config_load(text: String) -> Config {
    let mut cfg: Config = config_new();
    let lines: Vec<String> = text.split("\n".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    let mut section: String = "".to_string();
    let mut i: i64 = 0_i64;
    while i < (lines.len() as i64) {
        let line: String = lines[(i) as usize].clone().trim().to_string();
        if (line.len() as i64) == 0_i64 {
            i = i + 1_i64;
        } else {
            if line.starts_with("#".to_string().as_str()) {
                i = i + 1_i64;
            } else {
                if line.starts_with("[".to_string().as_str()) && line.ends_with("]".to_string().as_str()) {
                    let inner: String = line.replace("[".to_string().as_str(), "".to_string().as_str()).replace("]".to_string().as_str(), "".to_string().as_str());
                    section = inner.trim().to_string();
                    i = i + 1_i64;
                } else {
                    let line_chs: Vec<String> = line.chars().map(|c| c.to_string()).collect::<Vec<String>>();
                    let mut eq_pos: i64 = -1_i64;
                    let mut ci: i64 = 0_i64;
                    while ci < (line_chs.len() as i64) {
                        if line_chs[(ci) as usize].clone() == "=".to_string() && eq_pos == -1_i64 {
                            eq_pos = ci;
                        };
                        ci = ci + 1_i64;
                    }
                    if eq_pos > 0_i64 {
                        let mut raw_key: String = "".to_string();
                        let mut raw_val: String = "".to_string();
                        let mut ci2: i64 = 0_i64;
                        while ci2 < eq_pos {
                            raw_key = format!("{}{}", raw_key, format!("{}", line_chs[(ci2) as usize].clone()));
                            ci2 = ci2 + 1_i64;
                        }
                        ci2 = eq_pos + 1_i64;
                        while ci2 < (line_chs.len() as i64) {
                            raw_val = format!("{}{}", raw_val, format!("{}", line_chs[(ci2) as usize].clone()));
                            ci2 = ci2 + 1_i64;
                        }
                        let key: String = raw_key.trim().to_string();
                        let val: String = raw_val.trim().to_string();
                        let mut full_key: String = key.clone();
                        if (section.len() as i64) > 0_i64 {
                            full_key = format!("{}.{}", section, key);
                        };
                        cfg = config_set(cfg.clone(), full_key.clone(), val.clone());
                    };
                    i = i + 1_i64;
                };
            };
        };
    }
    return cfg;
}

fn config_save(cfg: Config) -> String {
    let mut root_keys: Vec<String> = vec![];
    let mut root_vals: Vec<String> = vec![];
    let mut sec_names: Vec<String> = vec![];
    let mut i: i64 = 0_i64;
    while i < (cfg.keys.len() as i64) {
        let key_i: String = cfg.keys[(i) as usize].clone();
        if key_i.contains(&".".to_string()) {
            let parts: Vec<String> = key_i.split(".".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
            let sec_nm: String = parts[(0_i64) as usize].clone();
            if !str_array_contains(sec_names.clone(), sec_nm.clone()) {
                sec_names.push(sec_nm.clone());
            };
        } else {
            root_keys.push(key_i.clone());
            root_vals.push(cfg.values[(i) as usize].clone());
        };
        i = i + 1_i64;
    }
    let mut out: String = "".to_string();
    let mut ri: i64 = 0_i64;
    while ri < (root_keys.len() as i64) {
        out = format!("{}{} = {}
", out, root_keys[(ri) as usize].clone(), root_vals[(ri) as usize].clone());
        ri = ri + 1_i64;
    }
    let mut si: i64 = 0_i64;
    while si < (sec_names.len() as i64) {
        let section_name: String = sec_names[(si) as usize].clone();
        out = format!("{}
[{}]
", out, section_name);
        let mut ki: i64 = 0_i64;
        while ki < (cfg.keys.len() as i64) {
            let key_ki: String = cfg.keys[(ki) as usize].clone();
            if key_ki.starts_with(format!("{}.", section_name).as_str()) {
                let subkey: String = key_ki.replace(format!("{}.", section_name).as_str(), "".to_string().as_str());
                out = format!("{}{} = {}
", out, subkey, cfg.values[(ki) as usize].clone());
            };
            ki = ki + 1_i64;
        }
        si = si + 1_i64;
    }
    return out;
}

fn main() {
    let text1: String = "# App config\napp_name = MyApp\n\n[database]\nhost = localhost\nport = 5432\n\n[server]\nhost = 0.0.0.0\nport = 8080".to_string();
    let cfg1: Config = config_load(text1.clone());
    println!("{}", format!("app_name = {}", config_get(cfg1.clone(), "app_name".to_string())));
    println!("{}", format!("database.host = {}", config_get(cfg1.clone(), "database.host".to_string())));
    println!("{}", format!("database.port = {}", config_get(cfg1.clone(), "database.port".to_string())));
    println!("{}", format!("server.port = {}", config_get(cfg1.clone(), "server.port".to_string())));
    println!("{}", format!("missing.key = {}", config_get(cfg1.clone(), "missing.key".to_string())));
    let cfg1a: Config = config_set(cfg1.clone(), "database.host".to_string(), "remotehost".to_string());
    println!("{}", format!("db.host after set = {}", config_get(cfg1a.clone(), "database.host".to_string())));
    println!("{}", format!("db.port unchanged = {}", config_get(cfg1a.clone(), "database.port".to_string())));
    let cfg1b: Config = config_set(cfg1a.clone(), "cache.ttl".to_string(), "300".to_string());
    println!("{}", format!("cache.ttl = {}", config_get(cfg1b.clone(), "cache.ttl".to_string())));
    let cfg2: Config = config_load("".to_string());
    println!("{}", format!("empty get = {}", config_get(cfg2.clone(), "any.key".to_string())));
    let cfg2a: Config = config_set(cfg2.clone(), "timeout".to_string(), "30".to_string());
    let cfg2b: Config = config_set(cfg2a.clone(), "app.name".to_string(), "Demo".to_string());
    println!("{}", format!("timeout = {}", config_get(cfg2b.clone(), "timeout".to_string())));
    println!("{}", format!("app.name = {}", config_get(cfg2b.clone(), "app.name".to_string())));
    let text3: String = "[db]\nhost = first\nhost = second".to_string();
    let cfg3: Config = config_load(text3.clone());
    println!("{}", format!("dup key = {}", config_get(cfg3.clone(), "db.host".to_string())));
    let text4: String = "[logging]\nfile.path = /var/log/app.log\nfile.level = DEBUG".to_string();
    let cfg4: Config = config_load(text4.clone());
    println!("{}", format!("logging.file.path = {}", config_get(cfg4.clone(), "logging.file.path".to_string())));
    println!("{}", format!("logging.file.level = {}", config_get(cfg4.clone(), "logging.file.level".to_string())));
}

