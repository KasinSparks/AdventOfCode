use crate::utils::{bytes_to_num::bytes_to_num, io::read_file_split_newlines};
use crate::utils::parsing::lexer::{Lexer, TokenType};

#[derive(Debug)]
struct Race {
    time: usize,     // milliseconds
    distance: usize, // millimeters
}

pub fn sln(input_path: &str) -> i32 {
    let input: Vec<String> = read_file_split_newlines(input_path);

    let mut data: Vec<Vec<usize>> = Vec::new();

    let mut races: Vec<Race> = Vec::new();

    for line in input {
        let lexer = Lexer::new(line);
        data.push(lexer.lex()
                        .unwrap()
                        .iter()
                        .filter(|token| token.get_token_type() == TokenType::NUMBER)
                        .map(|token| bytes_to_num(token.get_data().as_bytes()).unwrap())
                        .collect()
        );
    }

    for i in 0..data[0].len() {
        races.push(
            Race {
                time: data[0][i],
                distance: data[1][i]
            }
        );
    }

    println!("Boats: {:?}", races);

    // simple sol
    let mut num_winning_times: Vec<usize> = Vec::new();

    for race in races {
        let total_time = race.time as f64;
        let total_dist= race.distance as i64;

        // quadratic formula
        let time_part = f64::sqrt((f64::powi(total_time, 2) - (4.0 * total_dist as f64)) as f64);
    
        // Trim the range so we are "inside" of the valid times
        let t_range = (((time_part + total_time) / 2.0).ceil() - 1.0, ((time_part - total_time) / 2.0).ceil());

        num_winning_times.push((t_range.0 + t_range.1) as usize);
    }

    let mut result = 1;

    for t in num_winning_times {
        result *= t;
    }

    return result as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_06/practice_input.txt"), 288);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_06/input.txt"), 114400);
    }
}