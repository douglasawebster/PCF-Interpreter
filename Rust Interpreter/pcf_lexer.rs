use std::slice::Iter;
use std::iter::Peekable;

use crate::pcf_token::Token;

pub fn get_token(iter: &mut Peekable<Iter<char>>) -> Option<Token> {
    let c = iter.next()?;
    return
        match c {
            '=' => {
                let c2 = iter.peek()?;
                match c2 {
                    '>' => {
                        iter.next();
                        Some(Token::FnArrow)
                    },
                    _ => Some(Token::Equal),
                }
            },
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            'A'..='z' => {
                let id = get_id(vec![*c], iter);
                match id.as_str() {
                    "if" => Some(Token::IfSmy),
                    "then" => Some(Token::ThenSym),
                    "else" => Some(Token::ElseSym),
                    "true" => Some(Token::TrueSym),
                    "false" => Some(Token::FalseSym),
                    "succ" => Some(Token::SuccSym),
                    "pred" => Some(Token::PredSym),
                    "iszero" => Some(Token::IsZeroSym),
                    "fn" => Some(Token::FnSym),
                    "rec" => Some(Token::RecSym),
                    "let" => Some(Token::LetSym),
                    "in" => Some(Token::InSym),
                    "end" => Some(Token::EndSym),
                    _ => Some(Token::Id(id)),
                }
            },
            '0'..='9' => {
                let num = get_num(iter, c.to_digit(10)?);
                Some(Token::Num(num))
            },
            ' ' | '\t' | '\n' | '\r' => get_token(iter),
            _ => {
                let mut error = String::from("Illegal Character: ");
                error.push(*c);
                Some(Token::Error(error))
            },
        }
}

fn get_id(mut characters: Vec<char>, iter: &mut Peekable<Iter<char>>) -> String {
    while let Some(c) = iter.peek() {
        match c {
            'A'..='z' => {
                characters.push(**c);
                iter.next();
            },
            _ => return characters.into_iter().collect(),
        }
    }
    return characters.into_iter().collect();
}

fn get_num(iter: &mut Peekable<Iter<char>>, num: u32) -> u32 {
    let mut number = num;
    while let Some(c) = iter.peek() {
        match c {
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap();
                number = (number*10) + digit;
                iter.next();
            },
            _ => return num,
        }
    }
    return number
}