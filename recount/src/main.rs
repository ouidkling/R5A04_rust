// https://open.kattis.com/problems/recount
use std::collections::HashMap;
use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let lines = input.lines();

    let mut candidates: HashMap<String, i32> = HashMap::new();

    for line in lines
    {
        let candidate = line.to_string();
        if candidate.eq(&"***".to_string()) {
            break
        }
        if candidates.get(&candidate).is_some() {
            let old_value = *candidates.get(&candidate).unwrap();
            let new_value = old_value + 1;
            candidates.remove(&candidate);
            candidates.insert(candidate, new_value);
        } else {
            candidates.insert(candidate, 1);
        }
    }

    let mut winner = (0, String::new());
    let mut second = (0, String::new());

    for (candidate, score) in candidates {
        if score >= winner.0 {
            second = winner.clone();
            winner = (score, candidate);
            continue;
        }

        if score > second.0 {
            second = (score, candidate);
            continue;
        }
    }

    if winner.0.eq(&second.0) {
        println!("Runoff!")
    } else {
        println!("{}", winner.1)
    }
}
