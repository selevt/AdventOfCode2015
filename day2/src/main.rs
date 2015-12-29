use std::env;
use std::cmp;

#[derive(Debug)]
struct Gift {
    w: i32,
    l: i32,
    h: i32,
}

impl Gift {
    // Returns the two smaller sides
    fn smaller_sides(&self) -> Vec<i32> {
        let mut sorted_sides = vec![self.w, self.l, self.h];
        sorted_sides.sort();
        let (first_two, _last) = sorted_sides.split_at(2);
        return first_two.to_vec();
    }
}

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let gifts = parse(x);
            let (required_paper, required_ribbon) = calculate_material(gifts);
            println!("The elves need {} wrapping paper", required_paper);
            println!("The elves need {} ribbon", required_ribbon);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }

}

fn parse(input: String) -> Vec<Gift> {
    let lines = input.lines();

    lines.map(|line| {
             let mut dims = line.split('x');
             Gift {
                 w: dims.next().unwrap().parse::<i32>().unwrap(),
                 l: dims.next().unwrap().parse::<i32>().unwrap(),
                 h: dims.next().unwrap().parse::<i32>().unwrap(),
             }


         })
         .collect()
}

fn calculate_material(gifts: Vec<Gift>) -> (i32, i32) {
    let mut res_paper = 0i32;
    let mut res_ribbon = 0i32;

    for gift in gifts {
        // wrapping paper
        let side_a = gift.l * gift.w;
        let side_b = gift.w * gift.h;
        let side_c = gift.h * gift.l;

        let smallest_side = cmp::min(cmp::min(side_a, side_b), side_c);

        let giftres = (2 * side_a) + (2 * side_b) + (2 * side_c) + smallest_side;
        res_paper += giftres;

        // ribbon
        let smaller_sides = gift.smaller_sides();
        res_ribbon += smaller_sides.iter().fold(0, |sum, side| sum + (2 * side)) +
                      (gift.l * gift.w * gift.h);
    }

    return (res_paper, res_ribbon);
}
