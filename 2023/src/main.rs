mod utils;
mod days;

//use crate::utils::io::read_file;
use crate::days::*;

fn main() {
    println!("Advent of Code");
    //let result = day_01::part1::sln("./src/days/day_01/input.txt");
    //let result = day_01::part2::sln("./src/days/day_01/input.txt");
    //let result = day_02::part1::sln("./src/days/day_02/input.txt");
    //let result = day_02::part2::sln("./src/days/day_02/input.txt");
    //let result = day_03::part1::sln("./src/days/day_03/input.txt");
    let result = day_03::part2::sln("./src/days/day_03/input.txt");
    println!("Result: {}", result);
}
