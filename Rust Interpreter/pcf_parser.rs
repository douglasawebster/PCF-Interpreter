use crate::pcf_token::Token;
use crate::pcf_expression::Expr;

pub fn parse_expressions(tokens: &[Token]) -> (Expr, &[Token]) {
    let (ast, rest) = parse_exp(tokens);
    match rest {
        [] | [Token::EOF] => (ast, &[]),
        _ => {
            parse_exp2(ast, rest)
        }
    }
}

fn parse_exp2(ast: Expr, tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens {
        [] | [Token::EOF] => (ast, &[]),
        [head,..] => {
            match head {
                Token::RParen
                | Token::FnArrow
                | Token::ThenSym
                | Token:: ElseSym
                | Token::InSym
                | Token::EndSym => (ast, tokens),
                _ => {
                    let (ast2, rest) = parse_exp(tokens);
                    parse_exp2(Expr::App { func: Box::new(ast), arg: Box::new(ast2) }, rest)
                }
            }
        }
    }
}

fn parse_exp(tokens: &[Token]) -> (Expr, &[Token]) {
    return
        match tokens  {
            [Token::Id(s),   tail @ ..] => (Expr::Id(s.clone()), tail),
            [Token::Num(n),   tail @ ..] => (Expr::Num(n.clone()), tail),
            [Token::TrueSym,        tail @ ..] => (Expr::Bool(true), tail),
            [Token::FalseSym,       tail @ ..] => (Expr::Bool(false), tail),
            [Token::SuccSym,        tail @ ..] => (Expr::Succ, tail),
            [Token::PredSym,        tail @ ..] => (Expr::Pred, tail),
            [Token::IsZeroSym,      tail @ ..] => (Expr::IsZero, tail),

            [Token::FnSym, Token::Id(s), Token::FnArrow, tail @ ..] => {
                let (body, rest) = parse_expressions(tail);
                (Expr::Func { param: s.clone(), body: Box::new(body) }, rest)
            },
            [Token::FnSym, Token::Id(_s), tail @ ..] => {
                (Expr::Error { error: String::from("=> expected after fn") }, tail)
            },
            [Token::FnSym, tail @ ..] => {
                (Expr::Error { error: String::from("Identifier expected after fn") }, tail)
            },

            [Token::RecSym, Token::Id(s), Token::FnArrow, tail @ ..] => {
                let (func, rest) = parse_expressions(tail);
                (Expr::Rec { func_name: s.clone(), body: Box::new(func) }, rest)
            },
            [Token::RecSym, Token::Id(_s), tail @ ..] => {
                (Expr::Error { error: String::from("=> expected after rec") }, tail)
            },
            [Token::RecSym, tail @ ..] => {
                (Expr::Error { error: String::from("Identifier expected after rec") }, tail)
            },

            [Token::LParen, tail @ ..] => {
                let (body, rest) = parse_expressions(tail);
                match rest {
                    [Token::RParen, tail @ ..] => (body, tail),
                    _ => (Expr::Error { error: String::from("Missing right paren") }, rest)
                }
            },

            [Token::IfSmy, tail @ ..] => {
                let (cond, rest) = parse_expressions(tail);
                match rest {
                    [Token::ThenSym, tail @ ..] => {
                        let (t_val, rest) = parse_expressions(tail);
                        match rest {
                            [Token::ElseSym, tail @ ..] => {
                                let (f_val, rest) = parse_expressions(tail);
                                (Expr::If { cond: Box::new(cond), t_val: Box::new(t_val), f_val: Box::new(f_val) }, rest)
                            }
                            _ => (Expr::Error { error: String::from("Missing else") }, &[])
                        }
                    }
                    _ => (Expr::Error { error: String::from("Missing then") }, &[])
                }
            },

            /*[Token::LetSym, Token::Id(s), Token::Equal, tail @ ..] => {
                match tail {
                    [Token::InSym, tail @ ..] => (Expr::Err{error: String::from("Missing value after let")}, tail),
                    _ => {
                        let (value, rest) = parse_exps(tail);
                        match rest {
                            [Token::InSym, Token::EndSym, tail @ ..] => (Expr::Err{error: String::from("Missing exp after let")}, tail),
                            [Token::InSym, tail @ ..] => {
                                let (exp, rest) = parse_exp(tail);
                                match rest {
                                    [Token::EndSym, tail @ ..] => (Expr::ASTLet(s.clone(), Box::new(value), Box::new(exp)), tail),
                                    _ => (Expr::Err{error: String::from("Missing end")}, tail),
                                }
                            }
                            _ => (Expr::Err{error: String::from("Missing in")}, tail)
                        }
                    }
                }
            },

            [Token::LetSym, Token::Id(_s), tail @ ..] => {
                (Expr::Err{error: String::from("= expected after let")}, tail)
            },

            [Token::LetSym, tail @ ..] => {
                (Expr::Err{error: String::from("Expected identifier after let")}, tail)
            },*/


            [Token::EOF] => (Expr::Error { error: String::from("Unexpected end of input") }, &[]),

            _ => (Expr::Error { error: String::from("Unrecognized token") }, &[])
        }
}
