extern crate regex;
extern crate rand;

use std::env;
use std::fmt;
use std::collections::HashSet;
use regex::Regex;
use rand::{thread_rng, sample};

// Variation of the travelling salesman problem.
// This solution is just taking random routes to find the shortest one.

#[derive(PartialEq, Eq, Hash)]
struct Travel {
    from: String,
    to: String,
    dist: i32,
}

impl fmt::Debug for Travel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to {} ({})", self.from, self.to, self.dist)
    }
}


fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let short_tour = walk(x);
            println!("{}", short_tour);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn walk(input: String) -> i32 {

    let r = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();

    let mut all_locations = HashSet::new();
    let mut available_travels = HashSet::new();

    for line in input.lines() {
        let caps = r.captures(line).unwrap();
        let from = caps.at(1).unwrap();
        let to = caps.at(2).unwrap();
        let dist = caps.at(3).unwrap();

        all_locations.insert(from.to_string());
        all_locations.insert(to.to_string());

        available_travels.insert(Travel {
            from: from.to_string(),
            to: to.to_string(),
            dist: dist.parse::<i32>().unwrap(),
        });
        available_travels.insert(Travel {
            from: to.to_string(),
            to: from.to_string(),
            dist: dist.parse::<i32>().unwrap(),
        });
    }

    println!("Havte to visit all of these: {:?}", all_locations);
    println!("Available travels: {:?}", available_travels);

    let mut shortest_travel = None;
    // Part 1
    // let mut shortest_route = std::i32::MAX;
    let mut shortest_route = std::i32::MIN;

    // Very clever algorithm. Just randomly try a couple of times
    let mut rng = thread_rng();

    'tries: for i in 1..100_000 {
        if i % 500 == 0 {
            println!(".");
        }
        let mut travels = vec![];
        let mut visited = HashSet::new();

        let mut last_loc = sample(&mut rng, &all_locations, 1)[0].to_string();

        visited.insert(last_loc.clone());

        // TODO: This depends on complete data. If something is missing,
        // this ends up in a endless loop. This can be avoided by eliminating the
        // used choices from the random pool (which would also optimize it)
        while visited.len() < all_locations.len() {
            let t = sample(&mut rng, &available_travels, 1)[0];
            // for t in available_travels.iter() {
            if t.from == last_loc && !visited.contains(&t.to) {
                last_loc = t.to.clone();
                visited.insert(last_loc.clone());
                travels.push(t);
            }
        }

        let mut dist = 0i32;
        for t in travels.iter() {
            dist = dist + t.dist;
        }

        // Part 1
        // if dist < shortest_route {
        //     println!("Found shorter route");
        if dist > shortest_route {
            println!("Found longer route");
            print_route(&travels);
            println!("Distance: {}", dist);

            shortest_route = dist;
            shortest_travel = Some(travels);
        }
    }

    match shortest_travel {
        None => panic!("No valid route found"),
        Some(t) => {
            println!("Shortest Route: {:?}", t);
            println!("Distance: {:?}", shortest_route);
            return shortest_route;
        }
    }

    return 0;
}

fn print_route(travels: &Vec<&Travel>) {
    print!("{} ", travels[0].from);
    let mut dist = 0i32;
    for t in travels.iter() {
        dist = dist + t.dist;
        print!("-> {} ", t.to);
    }
    println!("");

}
