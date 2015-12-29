extern crate regex;

use regex::Regex;
use std::env;
use std::cmp;

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let num_lights = play_with_the_lights(x);
            println!("{}", num_lights);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn play_with_the_lights(input: String) -> i32 {
    // Initialize lights, all powered off
    let mut lights = init_lights();

    let instruction = Regex::new(r"(.+) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    for line in input.lines() {
        match instruction.captures(line) {
            None => println!("Invalid line sadface"),
            Some(cap) => {
                // TODO: this should go into a function
                // the unwraps are fine because of restrictions in the regex
                let task = cap.at(1).unwrap();
                let pos1_x = cap.at(2).unwrap().parse::<usize>().unwrap();
                let pos1_y = cap.at(3).unwrap().parse::<usize>().unwrap();
                let pos2_x = cap.at(4).unwrap().parse::<usize>().unwrap();
                let pos2_y = cap.at(5).unwrap().parse::<usize>().unwrap();

                for x in pos1_x..(pos2_x + 1) {
                    for y in pos1_y..(pos2_y + 1) {
                        let current_brightness = lights[x][y];
                        match task {
                            "turn on" => lights[x][y] = current_brightness + 1,
                            "turn off" => lights[x][y] = cmp::max(0, current_brightness - 1),
                            "toggle" => lights[x][y] = current_brightness + 2,
                            _ => println!("Unknown instruction {}", task),
                        }
                    }
                }
            }
        }
    }

    // TODO: there's probably a nice functional way to do this
    let mut num_shinies = 0i32;
    for row in lights {
        for light in row {
            num_shinies = num_shinies + light;
        }
    }

    return num_shinies;
}

fn init_lights() -> Vec<Vec<i32>> {
    let mut lights = vec![];
    for _ in 0..1000 {
        let mut row = vec![];
        for _ in 0..1000 {
            row.push(0);
        }

        lights.push(row);
    }

    return lights;
}
