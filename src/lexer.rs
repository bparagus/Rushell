use std::{iter::Peekable, str::Chars};

use TokenType::*;

pub fn lex(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut it = line.chars().peekable();
    let mut value = String::new();

    while let Some(char) = it.next() {
        match char {
            '&' => {
                let opt = LexerOpt::new("background token", AMPERSAND, "and", AND);
                generate_repeatable_token(opt, '&', &mut it, &mut value, &mut tokens);
            }
            '|' => {
                let opt = LexerOpt::new("pipe", PIPE, "or", OR);
                generate_repeatable_token(opt, '|', &mut it, &mut value, &mut tokens);
            }
            '<' => {
                let opt = LexerOpt::new("input redirection", LESS, "heredoc", LESSLESS);
                generate_repeatable_token(opt, '<', &mut it, &mut value, &mut tokens);
            }
            '>' => {
                let opt = LexerOpt::new(
                    "output redirection",
                    GREAT,
                    "output redirection - append",
                    GREATGREAT,
                );
                generate_repeatable_token(opt, '>', &mut it, &mut value, &mut tokens);
            }
            '"' => {
                // Make it work first, and then refactor it into a function
                value.push(char);
                while let Some(c) = it.next() {
                    value.push(c);
                    if c == '"' {
                        break;
                    }
                }
                // tokens.clear();
                // tokens.push(Token::new("Unclosed doublequotes", ERROR));
            }
            ';' => generate_token("semicolon", SEMICOLON, &mut value, &mut tokens),
            ' ' => push_word(&mut value, &mut tokens),
            _ => value.push(char),
        }
    }

    push_word(&mut value, &mut tokens);
    tokens
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    WORD,
    PIPE,
    AMPERSAND,
    LESS,
    GREAT,
    LESSLESS,
    GREATGREAT,
    SEMICOLON,
    OR,
    AND,
    ERROR,
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub ttype: TokenType,
}

impl Token {
    fn new(literal: &str, ttype: TokenType) -> Token {
        Token {
            literal: String::from(literal),
            ttype,
        }
    }
}

struct LexerOpt {
    single_literal: String,
    single_type: TokenType,
    repeat_literal: String,
    repeat_type: TokenType,
}

impl LexerOpt {
    fn new(
        single_literal: &str,
        single_type: TokenType,
        repeat_literal: &str,
        repeat_type: TokenType,
    ) -> LexerOpt {
        LexerOpt {
            single_literal: String::from(single_literal),
            single_type,
            repeat_literal: String::from(repeat_literal),
            repeat_type,
        }
    }
}

fn generate_repeatable_token(
    opt: LexerOpt,
    char_check: char,
    it: &mut Peekable<Chars>,
    value: &mut String,
    tokens: &mut Vec<Token>,
) {
    if it.peek().is_some() && *it.peek().unwrap() == char_check {
        generate_token(&opt.repeat_literal, opt.repeat_type, value, tokens);
        it.next();
    } else {
        generate_token(&opt.single_literal, opt.single_type, value, tokens);
    }
}

fn generate_token(literal: &str, ttype: TokenType, value: &mut String, tokens: &mut Vec<Token>) {
    push_word(value, tokens);
    tokens.push(Token::new(literal, ttype));
}

fn push_word(value: &mut String, tokens: &mut Vec<Token>) {
    if !value.is_empty() {
        tokens.push(Token::new(value, WORD))
    }

    value.clear()
}
