mod utils;
mod days;

//use crate::utils::io::read_file;
use crate::days::*;

use crate::utils::perf::timing::Timer;


fn main() {
    println!("Advent of Code");
    let mut timer = Timer::new("Timer1");
    timer.start();
    //let result = day_01::part1::sln("./src/days/day_01/input.txt");
    //let result = day_01::part2::sln("./src/days/day_01/input.txt");
    //let result = day_02::part1::sln("./src/days/day_02/input.txt");
    //let result = day_02::part2::sln("./src/days/day_02/input.txt");
    //let result = day_03::part1::sln("./src/days/day_03/input.txt");
    //let result = day_03::part2::sln("./src/days/day_03/input.txt");
    //let result = day_04::part1::sln("./src/days/day_04/input.txt");
    //let result = day_04::part2::sln("./src/days/day_04/input.txt");
    //let result = day_05::part1::sln("./src/days/day_05/input.txt");
    //let result = day_05::part2::sln("./src/days/day_05/input.txt");
    //let result = day_05::part2_copy::sln("./src/days/day_05/input.txt");
    //let result = day_06::part1::sln("./src/days/day_06/input.txt");
    //let result = day_06::part2::sln("./src/days/day_06/input.txt");
    //let result = day_07::part1::sln("./src/days/day_07/input.txt");
    //let result = day_07::part2::sln("./src/days/day_07/input.txt");
    //let result = day_08::part1::sln("./src/days/day_08/input.txt");
    //let result = day_08::part2::sln("./src/days/day_08/input.txt");
    //let result = day_09::part1::sln("./src/days/day_09/input.txt");
    //let result = day_09::part2::sln("./src/days/day_09/input.txt");
    //let result = day_10::part1::sln("./src/days/day_10/input.txt");
    //let result = day_10::part2::sln("./src/days/day_10/input.txt");
    //let result = day_11::part1::sln("./src/days/day_11/input.txt");
    //let result = day_11::part2::sln("./src/days/day_11/input.txt", 1_000_000);
    let result = day_12::part1::sln("./src/days/day_12/input.txt");
    timer.end();
    println!("{}", timer);
    println!("Result: {}", result);
}