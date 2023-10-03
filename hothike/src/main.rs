use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    lines.next();

    let raw_vacation = lines.next().map(|x| x.to_string()).unwrap();
    let raw_vacation2 = raw_vacation.split_whitespace().map(|x| x.parse::<i32>().expect("Entier")).collect::<Vec<i32>>();
    let vacation = raw_vacation2.windows(3);

    let mut best_day = 0;
    let mut min_max_temp = 41;
    let mut count = 0;

    for trip in vacation {
        count += 1;
        let max_temp = trip.iter().max().unwrap();
        if max_temp < &min_max_temp {
            min_max_temp = *max_temp;
            best_day = count;
        }
    }
    println!("{} {}", best_day, min_max_temp);
}

