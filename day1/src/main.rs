use std::env;

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let floor = walk(x);
            println!("{}", floor);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn walk(input: String) -> i32 {

    let mut floor = 0i32;
    let mut entered_basement = false;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => println!("Invalid character {}", c),
        }

        if !entered_basement && floor == -1 {
            println!("Entered basement while on {}", (i + 1));
            entered_basement = true;
        }
    }

    return floor;
}
