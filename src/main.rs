mod common;
mod day_1;
mod day_2;
mod day_3;

use std::fs;

const DAY_1: bool = false;
const DAY_2: bool = false;
const DAY_3: bool = true;

fn main() {
    if DAY_1 {
        let sol_1_1 = crate::day_1::part_1(fs::File::open("./data/day_1.txt").unwrap());
        let sol_1_2 = crate::day_1::part_2(fs::File::open("./data/day_1.txt").unwrap());
        println!("Day One: Part1 = {}; Part 2 = {};", sol_1_1, sol_1_2);
    }

    if DAY_2 {
        let sol_2_1 = crate::day_2::part_1(fs::File::open("./data/day_2.txt").unwrap());
        let sol_2_2 = crate::day_2::part_2(fs::File::open("./data/day_2.txt").unwrap());
        println!("Day Two: Part1 = {}; Part 2 = {};", sol_2_1, sol_2_2);
    }

    if DAY_3 {
        let sol_3_1 = crate::day_3::part_1(fs::File::open("./data/day_3.txt").unwrap());
        let sol_3_2 = crate::day_3::part_2(fs::File::open("./data/day_3.txt").unwrap());
        println!("Day Three: Part1 = {}; Part 2 = {};", sol_3_1, sol_3_2);
    }
}
