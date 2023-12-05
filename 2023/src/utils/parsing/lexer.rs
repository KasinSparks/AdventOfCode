/* Kasin Sparks (kasinsparks@gmail.com) 2023 */

/// Lex a char stream input into tokens based on simple grammer

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    data: String,
}

impl Token {
    pub fn get_token_type(&self) -> TokenType {
        return self.token_type;
    }

    pub fn get_data(&self) -> &str {
        return self.data.as_str();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    WORD,
    NUMBER,
    SEMICOLON,
    COLON,
    WHITESPACE,
    NEWLINE,
    DOT,
    COMMA,
    PIPE,
    UNKNOWN,
}

#[derive(Debug)]
pub struct LexErr {
    msg: String,
}

pub struct Lexer {
    data: String,
}

impl Lexer {
    pub fn new(data: String) -> Self {
        return Lexer {
            data: data
        };
    }

    pub fn lex(&self) -> Result<Vec<Token>, LexErr> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut temp_buf: String = String::from("");
        let mut last_token_type: TokenType = TokenType::UNKNOWN;

        for c in self.data.chars() {
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
                if c == '\n' {
                    curr_token_type = TokenType::NEWLINE;
                } else {
                    curr_token_type = TokenType::WHITESPACE;
                }
            } else if c == ',' {
                curr_token_type = TokenType::COMMA;
            } else if c == '.' {
                curr_token_type = TokenType::DOT;
            } else if c == ';' {
                curr_token_type = TokenType::SEMICOLON;
            } else if c == ':' {
                curr_token_type = TokenType::COLON;
            } else if c == '|' {
                curr_token_type = TokenType::PIPE;
            } else {
                curr_token_type = TokenType::UNKNOWN;
                //return Err(LexErr { msg: format!("Unable to convert: {} into a token.", c) });
            }

            // TODO: Change how it gets the first token
            if last_token_type == TokenType::UNKNOWN {
                // skip
            } else if curr_token_type != last_token_type {
                tokens.push(Token {token_type: last_token_type, data: temp_buf.clone()});
                temp_buf.clear();
            }
            temp_buf.push(c);

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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let lexer = Lexer::new(String::from("test"));
        let result = lexer.lex().unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test2() {
        let lexer = Lexer::new(String::from("test \ntest2 42"));
        let result = lexer.lex().unwrap();
        // length 
        assert_eq!(result.len(), 7);
        
        // type
        assert_eq!(result[0].token_type, TokenType::WORD);
        assert_eq!(result[1].token_type, TokenType::WHITESPACE);
        assert_eq!(result[2].token_type, TokenType::NEWLINE);
        assert_eq!(result[3].token_type, TokenType::WORD);
        assert_eq!(result[4].token_type, TokenType::NUMBER);
        assert_eq!(result[5].token_type, TokenType::WHITESPACE);
        assert_eq!(result[6].token_type, TokenType::NUMBER);

        // data
        assert_eq!(result[0].data, String::from("test"));
        assert_eq!(result[1].data, String::from(" "));
        assert_eq!(result[2].data, String::from("\n"));
        assert_eq!(result[3].data, String::from("test"));
        assert_eq!(result[4].data, String::from("2"));
        assert_eq!(result[5].data, String::from(" "));
        assert_eq!(result[6].data, String::from("42"));
    }

    #[test]
    fn test3() {
        let lexer = Lexer::new(String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"));
        let result = lexer.lex().unwrap();
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