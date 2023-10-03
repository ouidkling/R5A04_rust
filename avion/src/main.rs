use std::io::{self, prelude::*};
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    let blimp_check = Regex::new(r"^([A-Z0-9-]){0,11}$").unwrap();

    let mut count = 0;
    let mut cia = String::new();
    for line in lines {
        count += 1;
        if blimp_check.captures(line).is_none() {
            panic!("Code invalide : {}", line);
        };

        if line.contains("FBI") {
            cia.push_str(&count.to_string());
            cia.push(' ');
        }
    }
    if cia.is_empty(){
        println!("HE GOT AWAY!");
    }
    else {
        println!("{}", cia.trim_end());
    }
}

