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
    //let result = day_06::part1::sln("./src/days/day_06/input.txt");
    let result = day_06::part2::sln("./src/days/day_06/input.txt");
    timer.end();
    println!("{}", timer);
    println!("Result: {}", result);
}