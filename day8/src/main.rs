use std::env;

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let (raw, parsed) = count_chars(x);
            println!("Raw characters: {}", raw);
            println!("Parsed Characters: {}", parsed);
            println!("Diff: {}", raw - parsed);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }
}

fn count_chars(input: String) -> (i32, i32) {
    let mut raw = 0i32;
    let mut parsed = 0i32;
    for line in input.lines() {
        raw = raw + (line.len() as i32);

        // fake parsing
        let mut p1 = line.to_string();
        p1 = p1.replace("\"", "__");
        p1 = p1.replace("\\", "__");
        // start and end quotes
        p1 = p1 + "__";

        parsed = parsed + (p1.len() as i32);
    }

    return (raw, parsed);
}
