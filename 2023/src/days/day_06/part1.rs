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
    let mut num_winning_times: Vec<u32> = Vec::new();

    for race in races {
        let mut num_won = 0;

        for i in 0..(race.time) {
            let speed = i;

            let distance = speed * ((race.time) - i);

            if distance > race.distance {
                num_won += 1;
            }
        }

        num_winning_times.push(num_won);
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

    /*
    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_06/input.txt"), 0);
    }
    */
}