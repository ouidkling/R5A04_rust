// https://open.kattis.com/problems/everywhere
use std::collections::HashSet;
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    let free_food = lines.next().unwrap().parse::<i32>().expect("Conversion en entier");

    let mut free_food_days: HashSet<i32> = HashSet::new();
    let mut count = 0;

    for _ in 0..free_food {
        let raw_day_range = lines.next().unwrap();
        let mut day_range = raw_day_range.split_whitespace().map(|x| x.parse::<i32>().ok());
        let first_day = day_range.next().unwrap().expect("Entier");
        let last_day = day_range.next().unwrap().expect("Entier");
        for day in first_day..last_day + 1 {
            if !free_food_days.contains(&day) {
                free_food_days.insert(day);
                count += 1;
            }
        }
    }
    println!("{}", count);
}
