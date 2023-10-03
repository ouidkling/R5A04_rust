use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();
    let raw_expenses = lines.next().map(|x| x.to_string()).unwrap();
    let expenses = raw_expenses.split_whitespace().map(|x| x.parse::<i32>().expect("Entier"));
    let sum_expenses = expenses.filter(|x| x < &0).sum::<i32>().abs();
    println!("{}", sum_expenses);
}

