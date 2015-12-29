extern crate regex;

use std::env;
use regex::Regex;


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

        let p = line.to_string();

        let no_quotes = &p[1..(line.len() - 1)];

        // fake parsing
        let mut p1 = no_quotes.to_string();
        p1 = p1.replace("\\\"", "_");
        p1 = p1.replace("\\\\", "_");
        let r = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
        p1 = r.replace_all(&p1, "_");

        parsed = parsed + (p1.len() as i32);
    }

    return (raw, parsed);
}
