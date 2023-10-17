// https://open.kattis.com/problems/cold
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();
    let raw_temperatures = lines.next().map(|x| x.to_string()).unwrap();
    let temperatures = raw_temperatures.split_whitespace().map(|x| x.parse::<i32>().expect("Entier"));
    let below_zero = temperatures.filter(|x| x < &0).count();
    println!("{}", below_zero);
}

