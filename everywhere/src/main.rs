// https://open.kattis.com/problems/everywhere
use std::collections::HashSet;
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    let test_cases = lines.next().unwrap().parse::<i32>().expect("Conversion en entier");
    for _ in 0..test_cases {
        let nb_cities = lines.next().unwrap().parse::<i32>().expect("Conversion en entier");
        let mut my_set: HashSet<String> = HashSet::new();
        let mut count = 0;
        for _ in 0..nb_cities {
            let city = lines.next().unwrap().to_string();
            if my_set.contains(city.as_str()) {
                continue;
            } else {
                my_set.insert(city.to_string());
                count += 1;
            }
        }
        println!("{}", count);
        drop(my_set);
    }
}
