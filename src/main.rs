mod day1;

fn main() {
    let day1 = include_str!("../inputs/day1.txt");
    println!("Day 1.1: Total fuel required for space ship alone: {}", day1::total_basic_fuel_of_modules(&day1));
    println!("Day 1.2: Total fuel required, including fuel for fuel: {}", day1::total_fuel_of_modules_incl_additional(&day1));
}
