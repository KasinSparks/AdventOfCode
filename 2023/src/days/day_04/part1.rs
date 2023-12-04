use crate::utils::io::read_file_split_newlines;
use crate::utils::parsing::lexer;
use crate::utils::bytes_to_num::bytes_to_num;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_nums: Vec<u32>,
    marked_nums:  Vec<u32>,
}

pub fn sln(input_path: &str) -> i32 {
    let input: Vec<String> = read_file_split_newlines(input_path);

    let mut cards: Vec<Card> = Vec::new();

    for line in input {
        let char_line: Vec<char> = line.chars().collect();
        if let Ok(tokens) = lexer::Lexer::lex(&char_line) {
            let mut curr_pos: usize = 0;
            let mut card: Card = Card {
                id: 0,
                winning_nums: Vec::new(),
                marked_nums: Vec::new()
            };

            // Go until :
            while curr_pos < tokens.len() && tokens[curr_pos].get_token_type() != lexer::TokenType::COLON {
                curr_pos += 1;
            }

            // Record card id
            card.id = bytes_to_num(tokens[curr_pos - 1].get_data().as_bytes()).unwrap();

            // Go until |
            while curr_pos < tokens.len() && tokens[curr_pos].get_token_type() != lexer::TokenType::PIPE{
                // Record the numbers
                if tokens[curr_pos].get_token_type() == lexer::TokenType::NUMBER {
                    card.winning_nums.push(bytes_to_num(tokens[curr_pos].get_data().as_bytes()).unwrap());
                }
                curr_pos += 1;
            }


            // Go until eol
            while curr_pos < tokens.len() {
                // Record the numbers
                if tokens[curr_pos].get_token_type() == lexer::TokenType::NUMBER {
                    card.marked_nums.push(bytes_to_num(tokens[curr_pos].get_data().as_bytes()).unwrap());
                }
                curr_pos += 1;
            }

            cards.push(card);
        }
    }

    //println!("Cards: {:?}", cards);
    let mut result: i32 = 0;

    // Check card
    for card in cards {
        let mut num_matched = 0;
        for num in card.marked_nums {
            if card.winning_nums.contains(&num) {
                num_matched += 1;
            }
        }

        if num_matched > 0 {
            result += i32::pow(2, num_matched - 1);
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_04/practice_input.txt"), 13);
    }
}