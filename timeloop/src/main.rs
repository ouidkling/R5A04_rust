// https://open.kattis.com/problems/timeloop
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let times = input.trim().parse::<i32>().expect("Entier");
    for i in 0..times {
        println!("{} Abracadabra", i + 1);
    }
}
