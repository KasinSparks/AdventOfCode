mod utils;
mod days;

//use crate::utils::io::read_file;
use crate::days::*;

fn main() {
    println!("Advent of Code");
    let result = day_01::part1::sln("./src/days/day_01/input.txt");
    println!("Result: {}", result);
}
