use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    let carrots = lines.next().expect("Lecture de la ligne 1")
        .split_whitespace().nth(1).expect("Lecture du nombre de carottes")
        .parse::<u32>().expect("Conversion du nombre de carottes");
    println!("{}", carrots);
}
