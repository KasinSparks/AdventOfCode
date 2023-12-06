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

    // simple sol
    let mut num_winning_times: Vec<usize> = Vec::new();

    for race in races {
        let mut num_won = 0;

        for i in 0..(race.time) as usize{
            let speed: usize = i;

            let distance: usize = speed * ((race.time as usize) - i);

            if distance > race.distance as usize {
                num_won += 1;
            }
        }

        num_winning_times.push(num_won);
    }

    let mut result: usize = 1;

    println!("num won: {}", num_winning_times[0]);

    for t in num_winning_times {
        result *= t;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_06/practice_input.txt"), 71503);
    }

    /*
    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_06/input.txt"), 0);
    }
    */
}