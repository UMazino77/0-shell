use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Command(String),  
    Flag(String),    
    Argument(String), 
    Semicolon,        
}

#[derive(Debug)]
pub enum LexerError {
    UnclosedQuote(usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnclosedQuote(pos) => write!(f, "Unclosed quote at position {}", pos),
        }
    }
}

pub type Result<T> = std::result::Result<T, LexerError>;

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.trim().to_string(),
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        if self.position < self.input.chars().count() {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut is_start_of_command = true;

        while self.current_char.is_some() {
            // Skip whitespace
            if self.current_char.unwrap().is_whitespace() {
                self.advance();
                continue;
            }

            let token = match self.current_char.unwrap() {
                ';' => {
                    self.advance();
                    Token::Semicolon
                }
                '\'' | '"' => {
                    let word = self.parse_quoted_string()?;
                    self.classify_word(word, is_start_of_command)
                }
                _ => {
                    let word = self.parse_word()?;
                    self.classify_word(word, is_start_of_command)
                }
            };

            is_start_of_command = match &token {
                Token::Semicolon => true, 
                _ => false,
            };

            tokens.push(token);
        }

        Ok(tokens)
    }

    fn classify_word(&self, word: String, is_start_of_command: bool) -> Token {
        if is_start_of_command {
            Token::Command(word)
        } else if word.starts_with('-') {
            Token::Flag(word)
        } else {
            Token::Argument(word)
        }
    }

    fn parse_quoted_string(&mut self) -> Result<String> {
        let quote_char = self.current_char.unwrap();
        let quote_start = self.position - 1;
        self.advance(); 
        let mut content = String::new();
        let mut escaped = false;

        while let Some(c) = self.current_char {
            if escaped {
                match c {
                   
                    '\\' => content.push('\\'),
                    '"' => content.push('"'),
                    '\'' => content.push('\''),
                    '0' => content.push('\0'),
                    // ' ' => content.push(' '),
                    '$' => content.push('$'), 
                    '`' => content.push('`'), 
                    _ => {
                        content.push('\\');
                        content.push(c);
                    }
                }

                escaped = false;
                self.advance();
            } else if c == '\\' {
                escaped = true;
                self.advance();
            } else if c == quote_char {
                self.advance();
                return Ok(content);
            } else {
                content.push(c);
                self.advance();
            }
        }

        Err(LexerError::UnclosedQuote(quote_start))
    }

    fn parse_word(&mut self) -> Result<String> {
        let mut word = String::new();
        let mut escaped = false;

        while let Some(c) = self.current_char {
            if escaped {
                word.push(c);
                escaped = false;
                self.advance();
            } else if c == '\\' {
                escaped = true;
                self.advance();
            } else if c.is_whitespace() || c == ';' {
                break;
            } else {
                word.push(c);
                self.advance();
            }
        }
        Ok(word)
    }
}

pub fn has_unclosed_quotes(input: &str) -> bool {
    let mut lexer = Lexer::new(input.to_string());
    lexer.tokenize().is_err()
}

pub fn tokenize_input(input: &str, user: &str) -> Vec<Vec<String>> {
    let mut lexer = Lexer::new(input.to_string());
    let mut result = Vec::new();
    let mut current_command = Vec::new();

    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                match token {
                    Token::Semicolon => {
                        if !current_command.is_empty() {
                            result.push(current_command);
                            current_command = Vec::new();
                        }
                    }
                    Token::Command(cmd) | Token::Flag(cmd) | Token::Argument(cmd) => {
                        let expanded = if cmd.starts_with('~') {
                            cmd.replacen("~", &format!("/home/{}", user), 1)
                        } else {
                            cmd
                        };

                        current_command.push(expanded);
                    }
                }
            }

            if !current_command.is_empty() {
                result.push(current_command);
            }
        }
        Err(e) => {
            eprintln!("Tokenization error: {}", e);
        }
    }

    result
}
