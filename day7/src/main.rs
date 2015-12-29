extern crate regex;

use regex::Regex;
use std::env;
use std::collections::HashMap;

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let res = eval_circuit(&x, "a");
            println!("{}", res);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn eval_circuit<'f>(input: &'f str, target: &'f str) -> String {
    let mut mapping = HashMap::new();


    let cmd = Regex::new(r"(.+) -> (.+)").unwrap();

    let lines = input.lines();
    for line in lines {
        match cmd.captures(line) {
            None => println!("Invalid line sadface: {}", line),
            Some(cap) => {
                // TODO: this should go into a function
                // the unwraps are fine because of restrictions in the regex
                let left = cap.at(1).unwrap();
                let right = cap.at(2).unwrap();

                mapping.insert(right, left);
            }
        }
    }

    // This is part 2
    mapping.insert("b", "3176");


    // TODO: this is less than optimal right now, just setting it at a few places
    let mut cache: HashMap<&'f str, i32> = HashMap::new();
    return eval_node(&mut cache, &mapping, target);
}


fn eval_node<'f>(cache: &mut HashMap<&'f str, i32>,
                 mappings: &HashMap<&'f str, &'f str>,
                 wire: &'f str)
                 -> String {
    let re_const = Regex::new(r"^(\d+)$").unwrap();
    let re_and = Regex::new(r"^(.+) AND (.+)$").unwrap();
    let re_or = Regex::new(r"^(.+) OR (.+)$").unwrap();
    let re_lshift = Regex::new(r"^(.+) LSHIFT (\d+)$").unwrap();
    let re_rshift = Regex::new(r"^(.+) RSHIFT (\d+)$").unwrap();
    let re_not = Regex::new(r"^NOT (.+)$").unwrap();

    let left;
    {
        // left = mappings.get(wire).unwrap().clone();
        left = match mappings.get(wire) {
            Some(v) => v.clone(),
            None => wire,
        };
    }

    match cache.get(wire) {
        Some(x) => return x.to_string(),
        None => {}
    }

    match re_const.captures(left) {
        Some(caps) => {
            let val = caps.at(1).unwrap().to_string();
            cache.insert(wire, val.parse::<i32>().unwrap());
            return val;
        }
        None => {}
    };

    match re_and.captures(left) {
        Some(caps) => {
            let v1 = caps.at(1).unwrap();
            let v2 = caps.at(2).unwrap();
            let res1 = eval_node(cache, mappings, v1).parse::<i32>().unwrap();
            let res2 = eval_node(cache, mappings, v2).parse::<i32>().unwrap();

            cache.insert(v1, res1);
            cache.insert(v2, res2);

            let x = res1 & res2;
            cache.insert(wire, x);
            return x.to_string();
        }
        None => {}
    };

    match re_or.captures(left) {
        Some(caps) => {
            let v1 = caps.at(1).unwrap();
            let v2 = caps.at(2).unwrap();
            let res1 = eval_node(cache, mappings, v1).parse::<i32>().unwrap();
            let res2 = eval_node(cache, mappings, v2).parse::<i32>().unwrap();

            cache.insert(v1, res1);
            cache.insert(v2, res2);

            let x = res1 | res2;
            cache.insert(wire, x);
            return x.to_string();
        }
        None => {}
    };

    match re_lshift.captures(left) {
        Some(caps) => {
            let v1 = caps.at(1).unwrap();
            let shift = caps.at(2).unwrap().parse::<i32>().unwrap();
            let res = eval_node(cache, mappings, v1).parse::<i32>().unwrap();

            let x = res << shift;
            cache.insert(wire, x);
            return x.to_string();
        }
        None => {}
    };

    match re_rshift.captures(left) {
        Some(caps) => {
            let v1 = caps.at(1).unwrap();
            let shift = caps.at(2).unwrap().parse::<i32>().unwrap();
            let res = eval_node(cache, mappings, v1).parse::<i32>().unwrap();

            let x = res >> shift;
            cache.insert(wire, x);
            return x.to_string();
        }
        None => {}
    };

    match re_not.captures(left) {
        Some(caps) => {
            let v1 = caps.at(1).unwrap();
            let res = eval_node(cache, mappings, v1).parse::<i32>().unwrap();

            let x = !res;
            cache.insert(wire, x);
            return x.to_string();
        }
        None => {}
    };

    return eval_node(cache, mappings, left);
}
