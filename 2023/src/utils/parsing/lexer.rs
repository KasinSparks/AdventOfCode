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
    NEWLINE,
    DOT,
    COMMA,
    PIPE,
    TAB,
    SPACE,
    STAR,
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

        let c_stream = self.data.as_str();
        let mut index: usize = 0;
        let mut c_ptr: usize = 0;

        // Kick it off
        // Lex all of the characters
        while let Some(next_char) = Self::peek(c_stream, &index) {

            match next_char {
                'a'..='z' | 'A'..='Z' => {
                    // Letters
                    while let Some(ch) = Self::peek(c_stream, &mut index) {
                        match ch {
                            'a'..='z' | 'A'..='Z' => {
                                Self::consume(c_stream, &mut index);
                            },
                            _ => { break; }
                        }
                    }

                    tokens.push(Token { token_type: TokenType::WORD, data: c_stream[c_ptr..index].to_string() });
                    c_ptr = index;
                },
                '0'..='9' => {
                    // Numbers
                    while let Some(ch) = Self::peek(c_stream, &mut index) {
                        match ch {
                            '0'..='9' => {
                                Self::consume(c_stream, &mut index);
                            },
                            _ => { break; }
                        }
                    }

                    tokens.push(Token { token_type: TokenType::NUMBER, data: c_stream[c_ptr..index].to_string() });
                    c_ptr = index;
                },
                '\n' | '\t' | ' ' |
                '\x21'..='\x2F' |
                '\x3A'..='\x40' |
                '\x5B'..='\x60' |
                '\x7B'..='\x7E' => {
                    // Whitespace chars and Symbols
                    let tt = match next_char {
                        '\n' => { TokenType::NEWLINE },
                        '\t' => { TokenType::TAB },
                        ' '  => { TokenType::SPACE },
                        '|' => { TokenType::PIPE},
                        ':' => { TokenType::COLON},
                        ';'  => { TokenType::SEMICOLON},
                        '.'  => { TokenType::DOT},
                        ','  => { TokenType::COMMA},
                        '*'  => { TokenType::STAR},
                        _    => { TokenType::UNKNOWN }
                    };

                    let c = Self::consume(c_stream, &mut index).unwrap();

                    tokens.push(Token { token_type: tt, data: String::from(c) });
                    c_ptr = index;
                },
                '\x00'..='\x1F' => {
                    // Ignore these control characters
                },
                _ => { todo!("Unmatched char: {}", next_char); }
            };

        }

        return Ok(tokens);
    }

    fn peek(s: &str, index: &usize) -> Option<char> {
        if let Some(ss) = s.get(*index..*index + 1) {
            return ss.chars().next();
        }

        return None;
    }

    fn consume(s: &str, index: &mut usize) -> Option<char> {
        let result = Self::peek(s, index);
        if result.is_some() {
            *index += 1;
        }
        return result;
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
        assert_eq!(result[1].token_type, TokenType::SPACE);
        assert_eq!(result[2].token_type, TokenType::NEWLINE);
        assert_eq!(result[3].token_type, TokenType::WORD);
        assert_eq!(result[4].token_type, TokenType::NUMBER);
        assert_eq!(result[5].token_type, TokenType::SPACE);
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

        //println!("{:#?}", result);

        // length 
        assert_eq!(result.len(), 43);

        let expected_tts = [
            TokenType::WORD,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::COLON,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
            TokenType::WORD,
            TokenType::SEMICOLON,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
            TokenType::WORD,
            TokenType::SEMICOLON,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
            TokenType::WORD,
            TokenType::COMMA,
            TokenType::SPACE,
            TokenType::NUMBER,
            TokenType::SPACE,
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