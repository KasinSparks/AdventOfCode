use crate::utils::{bytes_to_num::bytes_to_num, io::read_file_split_newlines};
use crate::utils::parsing::lexer::{Lexer, TokenType};

#[derive(Debug)]
struct Race {
    time: usize,     // milliseconds
    distance: usize, // millimeters
}

pub fn sln(input_path: &str) -> usize {
    let input: Vec<String> = read_file_split_newlines(input_path);

    //let mut data: Vec<Vec<u32>> = Vec::new();
    let mut data: Vec<usize> = Vec::new();

    let mut races: Vec<Race> = Vec::new();

    for line in input {
        let lexer = Lexer::new(line);
        let tokens = lexer.lex().unwrap();
        let temp: Vec<&str> = tokens.iter()
                                    .filter(|token| token.get_token_type() == TokenType::NUMBER)
                                    .map(|token| token.get_data())
                                    .collect();

        let mut temp_str = String::new();
        for s in temp {
            temp_str.push_str(s);
        }

        data.push(bytes_to_num(temp_str.as_bytes()).unwrap());
    }

    races.push(
        Race {
            time: data[0],
            distance: data[1]
        }
    );

    println!("Boats: {:?}", races);

    let mut result: usize = 0;

    //let total_time = races[0].time as i64;
    let total_time = races[0].time as f64;
    let total_dist= races[0].distance as i64;

    // quadratic formula
    let time_part = f64::sqrt((f64::powi(total_time, 2) - (4.0 * total_dist as f64)) as f64);

    // Trim the range so we are "inside" of the valid times
    let t_range = (((time_part + total_time) / 2.0).ceil() - 1.0, ((time_part - total_time) / 2.0).ceil());

    result = (t_range.0 + t_range.1) as usize;

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_06/practice_input.txt"), 71503);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_06/input.txt"), 21039729);
    }
}