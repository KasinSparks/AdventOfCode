/* Kasin Sparks (kasinsparks@gmail.com) 2023 */

/// Lex a char stream input into tokens based on simple grammer

pub struct Token {
    token_type: TokenType,
    data: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    WORD,
    LETTER,
    NUMBER,
    SEMICOLON,
    COLON,
    WHITESPACE,
    DOT,
    COMMA,
    UNKNOWN,
}

#[derive(Debug)]
pub struct LexErr {
    msg: String,
}

pub struct Lexer {}

impl Lexer {
    pub fn lex(char_stream: &[char]) -> Result<Vec<Token>, LexErr> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut temp_buf: String = String::from("");
        let mut curr_pos: usize = 0;
        let mut last_token_type: TokenType = TokenType::UNKNOWN;

        while curr_pos < char_stream.len() {
            let c = char_stream[curr_pos];
            //let mut curr_token_type: TokenType = TokenType::UNKNOWN;
            let curr_token_type: TokenType;

            // Only handle ascii right now
            if !c.is_ascii() {
                return Err(LexErr {msg: String::from("Not an ascii character")});
            }

            if c.is_ascii_alphabetic() {
                curr_token_type = TokenType::WORD;
            } else if c.is_ascii_digit() {
                curr_token_type = TokenType::NUMBER;
            } else if c.is_ascii_whitespace() {
                curr_token_type = TokenType::WHITESPACE;
            } else if c == ',' {
                curr_token_type = TokenType::COMMA;
            } else if c == '.' {
                curr_token_type = TokenType::DOT;
            } else if c == ';' {
                curr_token_type = TokenType::SEMICOLON;
            } else if c == ':' {
                curr_token_type = TokenType::COLON;
            } else {
                return Err(LexErr { msg: String::from("Unknown token type") });
            }

            let cc = Lexer::consume(char_stream, &mut curr_pos);
            match cc {
                Ok(c) => {
                    // TODO: Change how it gets the first token
                    if last_token_type == TokenType::UNKNOWN {
                        // skip
                    } else if curr_token_type != last_token_type {
                        tokens.push(Token {token_type: last_token_type, data: temp_buf.clone()});
                        temp_buf.clear();
                    }
                    temp_buf.push(c);
                },
                Err(e) => {return Err(e);}
            };

            last_token_type = curr_token_type;
        }

        // Check if we need to flush buffer
        if !temp_buf.is_empty() {
            tokens.push(Token {token_type: last_token_type, data: temp_buf.clone()});
            temp_buf.clear();
        }

        // TODO
        return Ok(tokens);
    }

    fn peek(char_stream: &[char], index: usize) -> Result<char, LexErr> {
        if index >= char_stream.len() {
            return Err(LexErr{ msg: String::from("index out of bounds.") });
        }
        return Ok(char_stream[index]);
    }

    fn consume(char_stream: &[char], index: &mut usize) -> Result<char, LexErr> {
        let result = Lexer::peek(char_stream, *index);
        *index += 1;

        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = ['t', 'e', 's', 't'];
        let result = Lexer::lex(&test).unwrap();
        assert_eq!(result.len(), 1);
        //assert_eq!(sln("./src/days/day_02/practice_input.txt"), 2286);
    }

    #[test]
    fn test2() {
        let test = ['t', 'e', 's', 't', ' ', 't', 'e', 's', 't', '2', ' ', '4', '2'];
        let result = Lexer::lex(&test).unwrap();
        // length 
        assert_eq!(result.len(), 6);
        
        // type
        assert_eq!(result[0].token_type, TokenType::WORD);
        assert_eq!(result[1].token_type, TokenType::WHITESPACE);
        assert_eq!(result[2].token_type, TokenType::WORD);
        assert_eq!(result[3].token_type, TokenType::NUMBER);
        assert_eq!(result[4].token_type, TokenType::WHITESPACE);
        assert_eq!(result[5].token_type, TokenType::NUMBER);

        // data
        assert_eq!(result[0].data, String::from("test"));
        assert_eq!(result[1].data, String::from(" "));
        assert_eq!(result[2].data, String::from("test"));
        assert_eq!(result[3].data, String::from("2"));
        assert_eq!(result[4].data, String::from(" "));
        assert_eq!(result[5].data, String::from("42"));
    }

    #[test]
    fn test3() {
        let test: Vec<char> = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".chars().collect();
        let result = Lexer::lex(&test).unwrap();
        // length 
        assert_eq!(result.len(), 43);

        let expected_tts = [
            TokenType::WORD,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::COLON,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
            TokenType::SEMICOLON,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
            TokenType::SEMICOLON,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::WHITESPACE,
            TokenType::NUMBER,
            TokenType::WHITESPACE,
            TokenType::WORD,
        ];

        for i in 0..result.len(){
            //println!("{}", i);
            assert_eq!(result[i].token_type, expected_tts[i]);
        }

        let expected_data = [
            "Game",
            " ",
            "4",
            ":",
            " ",
            "1",
            " ",
            "green",
            ",",
            " ",
            "3",
            " ",
            "red",
            ",",
            " ",
            "6",
            " ",
            "blue",
            ";",
            " ",
            "3",
            " ",
            "green",
            ",",
            " ",
            "6",
            " ",
            "red",
            ";",
            " ",
            "3",
            " ",
            "green",
            ",",
            " ",
            "15",
            " ",
            "blue",
            ",",
            " ",
            "14",
            " ",
            "red",
        ];
        
        // data
        for i in 0..result.len() {
            assert_eq!(result[i].data, expected_data[i]);
        }
    }
}