#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 9.0 / 5.0 + 32.0;
}

fn main() {
    println!("{}", format!("{}", celsius_to_fahrenheit(0.0)));
    println!("{}", format!("{}", celsius_to_fahrenheit(100.0)));
    println!("{}", format!("{}", celsius_to_fahrenheit(37.0)));
    println!("{}", format!("{}", celsius_to_fahrenheit(-273.15)));
    println!("{}", format!("{}", celsius_to_fahrenheit(-40.0)));
    println!("{}", format!("{}", celsius_to_fahrenheit(20.0)));
    println!("{}", format!("{}", celsius_to_fahrenheit(-10.0)));
}

