use std::env;
use std::collections::HashSet;

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            // Santa
            //let starting_positions = vec!((0,0));
            // Santa and Robot Santa
            let starting_positions = vec!((0,0), (0,0));
            let delivered_presents = deliver_presents(starting_positions, x);
            println!("{}", delivered_presents);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn deliver_presents(starting_positions: Vec<(i32, i32)>, input: String) -> u32 {
    let mut positions = starting_positions.clone();

    let mut happy_houses = HashSet::new();

    // Add the starting positions
    for starting_position in starting_positions {
        happy_houses.insert(starting_position);
    }

    let num_agents = positions.len();
    // Now let each agent walk in turns
    for (i, c) in input.chars().enumerate() {
        let active_agent = i % num_agents;
        let new_position = walk(positions[active_agent], c);
        positions[active_agent] = new_position;
        happy_houses.insert(new_position);
    }

    return happy_houses.len() as u32;
}

fn walk(current_position: (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '>' => (current_position.0 + 1, current_position.1),
        '<' => (current_position.0 - 1, current_position.1),
        '^' => (current_position.0, current_position.1 + 1),
        'v' => (current_position.0, current_position.1 - 1),
        _ => {
            println!("Invalid character {}", direction);
            current_position
        },
    }
}
