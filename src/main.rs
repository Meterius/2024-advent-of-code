mod common;
mod day_one;

use std::fs;

const DAY_ONE: bool = false;

fn main() {
    if DAY_ONE {
        let sol_day_one_part_1 = crate::day_one::part_1(fs::File::open("./data/day_1.txt").unwrap());
        let sol_day_one_part_2 = crate::day_one::part_2(fs::File::open("./data/day_1.txt").unwrap());
        println!("Day One: Part1 = {}; Part 2 = {};", sol_day_one_part_1, sol_day_one_part_2);
    }
}
