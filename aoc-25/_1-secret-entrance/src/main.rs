const INPUT: &str = include_str!("../input.txt");
const MAX_DIAL: i32 = 100;
const DIAL_SECRET_POS: i32 = 0;

fn main() {
    let mut password = 0;
    let mut dial = 50;
    let mut is_dial_at_zero = dial == 0;

    for line in INPUT.lines() {
        let mut count = line[1..].parse::<i32>().unwrap();

        // we count full turns since they always `click` at 0
        let full_turns = count / MAX_DIAL;
        password += full_turns;

        count %= MAX_DIAL;
        match line.chars().next().unwrap() {
            'L' => {
                dial -= count;
            }
            'R' => {
                dial += count;
            }
            _ => panic!("Wrong input.txt"),
        };

        // we check if `dial < 0 || MAX_DIAL <= dial` because that means that
        // the dial `clicked` at 0 only if it is not 0
        if !(0..=MAX_DIAL).contains(&dial) && !is_dial_at_zero {
            password += 1;
        }

        dial = dial.rem_euclid(MAX_DIAL);
        is_dial_at_zero = dial == 0;
        if dial == DIAL_SECRET_POS {
            password += 1;
        }
    }
    println!("{password}")
}
