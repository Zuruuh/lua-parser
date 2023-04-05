use super::token::{Token, TokenKind};

pub struct Lexer {
    token_buffer: Vec<Token>,
    current_block_buffer: String,
    current_block: Option<TokenKind>,
}

impl Lexer {
    pub fn lex(&mut self, content: String) -> Result<&Vec<Token>, String> {
        let lines = content.lines();
        let mut line_index = 0;
        let mut column_index = 0;

        for line in lines.peekable() {
            let chars = line.chars();
            let mut peekable_chars = chars.clone().peekable();
            line_index += 1;

            for current_char in chars {
                column_index += 1;
                peekable_chars.next();

                match current_char {
                    ' ' => match self.current_block {
                        Some(TokenKind::String(_)) => {
                            self.current_block_buffer.push(current_char);
                        }

                        Some(TokenKind::Identifier) => {
                            self.create_token_and_clear_buffer()?;
                        }
                        _ => {}
                    },
                    '=' => match self.current_block {
                        Some(TokenKind::String(_)) => {
                            self.current_block_buffer.push(current_char);
                        }

                        Some(TokenKind::Identifier) => {
                            self.create_token_and_clear_buffer()?;
                        }
                        _ => {
                            self.clear_block_buffer();
                            self.current_block_buffer.push(current_char);
                            self.current_block = Some(TokenKind::Assignement);
                            self.create_token_and_clear_buffer()?;
                        }
                    },
                    '"' | '\'' => match self.current_block {
                        Some(TokenKind::String(opening_string_char)) => {
                            // Allow \" to match only with \" and vice-versa
                            if opening_string_char != current_char {
                                continue;
                            }

                            let last_char = self.current_block_buffer.chars().nth_back(0);

                            // Previous char is an escape, continue string
                            if last_char == Some('\\') {
                                self.current_block_buffer.push(current_char);

                                continue;
                            }

                            // Current char is an end of string, clear buffer & create a new token
                            self.create_token_and_clear_buffer()?;
                        }
                        _ => {
                            self.current_block = Some(TokenKind::String(current_char));
                        }
                    },
                    '(' | ')' => match self.current_block {
                        Some(TokenKind::String(_)) => {
                            self.current_block_buffer.push(current_char);
                        }

                        None => {
                            self.current_block_buffer.push(current_char);
                            self.current_block = Some(match current_char {
                                '(' => TokenKind::OpeningParenthesis,
                                ')' => TokenKind::ClosingParenthesis,
                                _ => panic!(), // imposible scenario
                            });

                            self.create_token_and_clear_buffer()?;
                        }

                        Some(TokenKind::Identifier) => {
                            // End the previous identifier token
                            self.create_token_and_clear_buffer()?;

                            self.current_block_buffer.push(current_char);
                            self.current_block = Some(match current_char {
                                '(' => TokenKind::OpeningParenthesis,
                                ')' => TokenKind::ClosingParenthesis,
                                _ => panic!(), // imposible scenario
                            });

                            self.create_token_and_clear_buffer()?;
                        }
                        _ => {
                            println!("No impl for type {:?}", self.current_block);
                            todo!();
                        }
                    },
                    _ => match self.current_block {
                        Some(TokenKind::String(_)) => {
                            self.current_block_buffer.push(current_char);
                        }

                        _ => {
                            if self.current_block == Some(TokenKind::Identifier)
                                && !current_char.is_alphabetic()
                            {
                                return Err(format!("{}:{} Cannot use a number as the first character of an identifier", line_index, column_index));
                            }

                            if !current_char.is_alphanumeric() {
                                return Err(format!(
                                    "{}:{} Only use alphanumeric chars for identifier names",
                                    line_index, column_index
                                ));
                            }

                            self.current_block = Some(TokenKind::Identifier);
                            self.current_block_buffer.push(current_char);

                            if self.current_block_buffer == String::from("local") {
                                println!("{}", peekable_chars.peek().unwrap_or(&'0'));
                                self.current_block = Some(TokenKind::Local)
                            }
                        }
                    },
                }
            }
        }

        Ok(&self.token_buffer)
    }

    pub fn new() -> Self {
        Self {
            token_buffer: Vec::new(),
            current_block_buffer: String::new(),
            current_block: None,
        }
    }

    fn create_token_and_clear_buffer(&mut self) -> Result<(), String> {
        if self.current_block.is_none() {
            return Err(String::from(
                "Cannot create a token if not token type has been defined!",
            ));
        }

        self.token_buffer.push(Token::new(
            self.current_block.clone().unwrap(),
            self.current_block_buffer.clone(),
        ));

        self.current_block = None;
        self.clear_block_buffer();

        Ok(())
    }

    fn clear_block_buffer(&mut self) {
        self.current_block_buffer = String::new();
    }
}
