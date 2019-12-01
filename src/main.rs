mod day1;

fn main() {
    let day1 = include_str!("../inputs/day1.txt");
    println!("Day 1: Total fuel required: {}", day1::total_fuel_of_modules(&day1))
}
