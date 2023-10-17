// https://open.kattis.com/problems/pet
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let lines = input.lines();

    let mut winner = i32::from(0);
    let mut winner_grade = i32::from(0);
    let mut count = i32::from(0);
    for line in lines {
        count += 1;
        let grades = line.split_whitespace().map(|x| x.parse::<i32>().expect("Parse grades"));
        let sum_grade = grades.sum::<i32>();
        if sum_grade > winner_grade {
            winner = count;
            winner_grade = sum_grade;
        }
    }
    println!("{} {}", winner, winner_grade);
}

