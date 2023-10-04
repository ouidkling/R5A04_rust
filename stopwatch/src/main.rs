use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut lines = input.lines();
    let pressed = lines.next().map(|x| x.parse::<i32>().expect("Parse de i32")).unwrap();

    if pressed % 2 == 0 {
        let mut seconds = 0;

        let raw_ticking = lines.next().map(|x| x.to_string()).unwrap();
        let raw_ticking2 = raw_ticking.split_whitespace().map(|x| x.parse::<i32>().expect("Entier")).collect::<Vec<i32>>();
        let ticking = raw_ticking2.chunks(2);

        for tick in ticking {
            seconds += tick[1] - tick[0];
        }

        println!("{}", seconds);
    } else {
        println!("still running");
    }
}

