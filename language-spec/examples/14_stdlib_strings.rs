#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let padded: String = "  hello world  ".to_string();
    let trimmed: String = padded.trim().to_string();
    println!("{}", format!("trim result: {}", trimmed));
    let msg: String = "Hello, Lang4LLM!".to_string();
    let upper: String = msg.to_uppercase();
    let lower: String = msg.to_lowercase();
    println!("{}", format!("to_upper: {}", upper));
    println!("{}", format!("to_lower: {}", lower));
    let sentence: String = "the quick brown fox".to_string();
    let found: bool = sentence.contains(&"quick".to_string());
    if found {
        println!("{}", "Found \'quick\' in sentence".to_string());
    } else {
        println!("{}", "Not found".to_string());
    };
    let sw: bool = sentence.starts_with("the".to_string().as_str());
    let ew: bool = sentence.ends_with("fox".to_string().as_str());
    if sw {
        println!("{}", "Starts with \'the\'".to_string());
    } else {
        println!("{}", "Does not start with \'the\'".to_string());
    };
    if ew {
        println!("{}", "Ends with \'fox\'".to_string());
    } else {
        println!("{}", "Does not end with \'fox\'".to_string());
    };
    let replaced: String = sentence.replace("fox".to_string().as_str(), "cat".to_string().as_str());
    println!("{}", format!("After replace: {}", replaced));
    let csv: String = "apple,banana,cherry".to_string();
    let parts: Vec<String> = csv.split(",".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
    println!("{}", format!("Parts count: {}", parts.len()));
    let rejoined: String = parts.join(" | ".to_string().as_str());
    println!("{}", format!("Rejoined: {}", rejoined));
    let word: String = "hello".to_string();
    let letters: Vec<String> = word.chars().map(|c| c.to_string()).collect::<Vec<String>>();
    println!("{}", format!("Char count: {}", letters.len()));
    println!("{}", format!("First char: {}", letters[0_i64 as usize]));
    return ();
}

