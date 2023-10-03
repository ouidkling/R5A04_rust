use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();

    let mut qaly = f64::from(0.0);
    for line in lines {
        let mut values = line.split_whitespace();
        let quality = values.next().map(|x| x.parse::<f64>().expect("Parse quality")).unwrap();
        let years = values.next().map(|x| x.parse::<f64>().expect("Parse years")).unwrap();
        qaly += quality * years;
    }
    println!("{:.3}", qaly);
}

