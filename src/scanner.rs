use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(Clone, Debug)]
pub enum TokenId {
    Text,
    Block,
    Variable,
}

impl Display for TokenId {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TokenId::Text => fmt.write_str("0"),
            TokenId::Block => fmt.write_str("1"),
            TokenId::Variable => fmt.write_str("2"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub id: TokenId,
    pub content: String,
}

enum States {
    Text,
    OpenBracket,
    Variable,
    CloseVariable,
    Expression,
    CloseExpression,
    Comment,
    CloseComment,
}

pub fn get_tokens(content: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut state = States::Text;
    
    let mut tmp = Token {id: TokenId::Text, content: String::new()};
    for ch in content.chars() {
        match state {
            States::Text => {
                match ch {
                    '{' => state = States::OpenBracket,
                    _ch => tmp.content.push(ch),
                }
            },
            States::OpenBracket => {
                match ch {
                    '{' => {
                        if tmp.content.len() > 0 {
                            tokens.push(tmp.clone());
                        }
                        tmp.id = TokenId::Variable;
                        tmp.content.clear();
                        state = States::Variable
                    },
                    '%' => {
                        if tmp.content.len() > 0 {
                            tokens.push(tmp.clone());
                        }
                        tmp.id = TokenId::Block;
                        tmp.content.clear();
                        state = States::Expression
                    },
                    '#' => {
                        if tmp.content.len() > 0 {
                            tokens.push(tmp.clone());
                        }
                        state = States::Comment
                    },
                    _ch => {
                        tmp.content.push('{');
                        tmp.content.push(ch)
                    },
                }
            },
            States::Variable => {
                match ch {
                    '}' => state = States::CloseVariable,
                    _ch => tmp.content.push(ch),
                }
            },
            States::CloseVariable => {
                match ch {
                    '}' => {
                        tokens.push(tmp.clone());
                        tmp.id = TokenId::Text;
                        tmp.content.clear();
                        state = States::Text
                    },
                    _ch => {
                        tmp.content.push('}');
                        tmp.content.push(ch);
                        state = States::Variable
                    },
                }
            }
            States::Expression => {
                match ch {
                    '%' => state = States::CloseExpression,
                    _ch => tmp.content.push(ch),
                }
            },
            States::CloseExpression => {
                match ch {
                    '}' => {
                        tokens.push(tmp.clone());
                        tmp.id = TokenId::Text;
                        tmp.content.clear();
                        state = States::Text
                    },
                    _ch => {
                        tmp.content.push('%');
                        tmp.content.push(ch);
                        state = States::Expression
                    },
                }
            }
            States::Comment => {
                match ch {
                    '#' => state = States::CloseComment,
                    _ch => {},
                }
            },
            States::CloseComment => {
                match ch {
                    '}' => {
                        tmp.id = TokenId::Text;
                        tmp.content.clear();
                        state = States::Text
                    },
                    _ch => {
                        state = States::Comment
                    },
                }
            }
        }
    }
    tokens.push(tmp);
    Ok(tokens)
}