extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::env;

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let (naughty, nice) = naughty_and_nice(&x);
            println!("Naughty: {}", naughty.len());
            println!("Nice {}", nice.len());
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn naughty_and_nice<'f>(input: &'f String) -> (Vec<&'f str>, Vec<&'f str>) {
    let mut naughty = vec![];
    let mut nice = vec![];

    for word in input.split_whitespace() {
        if is_nice_part2(word) {
            nice.push(word);
        } else {
            naughty.push(word);
        }
    }

    return (naughty, nice);
}

fn is_nice_part1(word: &str) -> bool {
    // Using regex. Slower than optimizing the conditions together, but fast enough for this
    let re1 = Regex::new(r"(.*[aeiou].*){3,}").unwrap();
    // as of this writing, the rust regex crate does not provide backreferences, so the
    // check is done manually.
    let re3 = Regex::new(r"ab|cd|pq|xy").unwrap();

    return re1.is_match(word) && has_repeated_letter(word) && !re3.is_match(word);
}

fn has_repeated_letter(word: &str) -> bool {
    let mut last_char = ' ';
    for c in word.chars() {
        if c == last_char {
            return true;
        } else {
            last_char = c;
        }
    }

    return false;
}

fn is_nice_part2(word: &str) -> bool {
    return has_sibling_twice(word) && has_repeated_letter_with_fill(word);
}

fn has_sibling_twice(word: &str) -> bool {
    let mut siblings = HashSet::new();

    let mut last_char = ' ';
    let mut last_combo = String::new();

    for c in word.chars() {
        if last_char != ' ' {
            let mut combo = last_char.to_string();
            combo.push(c);

            if combo != last_combo && siblings.contains(&combo) {
                return true;
            } else {
                last_combo = combo.clone();
                siblings.insert(combo);
            }

        }

        last_char = c;
    }
    return false;
}

fn has_repeated_letter_with_fill(word: &str) -> bool {
    let mut cmp_char = ' ';
    let mut filler_char = ' ';

    for c in word.chars() {
        if c == cmp_char {
            return true;
        } else {
            cmp_char = filler_char;
            filler_char = c;
        }
    }

    return false;
}
