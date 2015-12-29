use std::env;
use std::char;

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let new_seq = look_and_say(x, 50);
            println!("{}", new_seq.len());
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn look_and_say(input: String, num_turns: i32) -> String {

    let mut val = input.clone();
    for _ in 0..num_turns {
        let mut turn_res = String::new();

        let mut last_digit = 'x';
        let mut repetitions = 0;

        for c in val.chars() {
            if c != last_digit {
                if repetitions > 0 {
                    turn_res.push(char::from_digit(repetitions, 10).unwrap());
                    turn_res.push(last_digit);
                }

                last_digit = c;
                repetitions = 1;
            } else {
                repetitions = repetitions + 1;
            }
        }

        // do it one more time for the trailing output
        turn_res.push(char::from_digit(repetitions, 10).unwrap());
        turn_res.push(last_digit);

        // set base value for next turn
        val = turn_res;
    }

    return val;
}
