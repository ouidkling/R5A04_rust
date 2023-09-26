use std::io::{self, prelude::*};

fn make_status(n: i32) -> String {
    if n % 2 == 0 {
        format!("{} is even", n)
    } else {
        format!("{} is odd", n)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();
    for line in lines {
        let n = line.parse::<i32>().expect("Entier");
        println!("{}", make_status(n));
    }
}