mod common;
mod day_one;
mod day_two;

use std::fs;

const DAY_ONE: bool = false;
const DAY_TWO: bool = true;

fn main() {
    if DAY_ONE {
        let sol_day_one_part_1 = crate::day_one::part_1(fs::File::open("./data/day_1.txt").unwrap());
        let sol_day_one_part_2 = crate::day_one::part_2(fs::File::open("./data/day_1.txt").unwrap());
        println!("Day One: Part1 = {}; Part 2 = {};", sol_day_one_part_1, sol_day_one_part_2);
    }

    if DAY_TWO {
        let sol_two_one_part_1 = crate::day_two::part_1(fs::File::open("./data/day_2.txt").unwrap());
        let sol_two_one_part_2 = crate::day_two::part_2(fs::File::open("./data/day_2.txt").unwrap());
        println!("Day One: Part1 = {}; Part 2 = {};", sol_two_one_part_1, sol_two_one_part_2);
    }
}
