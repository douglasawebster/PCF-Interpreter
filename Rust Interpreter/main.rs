mod pcf_lexer;
mod pcf_parser;
mod pcf_visitor;
mod pcf_token;
mod pcf_expression;
mod pcf_value;

use std::collections::HashMap;

use crate::pcf_token::Token;
use crate::pcf_expression::Expr;
use crate::pcf_value::Value;
use crate::pcf_visitor::InterpreterVisitor;

fn main() {
    // PCF code to multiply two numbers
    let code = String::from(
        "(fn x => \
                fn y => \
                    (rec mult => \
                        fn x => \
                            fn y => \
                                fn accum =>
                                    if (iszero x) \
                                        then accum
                                        else mult \
                                                (pred x) \
                                                y
                                                ((rec sum => fn x => fn y => if (iszero x) then y else sum (pred x) (succ y)) y accum)) x y 0
            ) 7 20"
    );

    let characters: Vec<char> = code.chars().collect();
    let tokens: Vec<Token> = get_tokens(characters.clone());
    let ast: Expr = parse(tokens.clone());
    let result: Result<Value, &str> = interpret(ast.clone());
    print_results(code, tokens, ast, result);
}

fn get_tokens(characters: Vec<char>) -> Vec<Token> {
    let mut iter = characters.iter().peekable();

    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = pcf_lexer::get_token(&mut iter);
        match token {
            Some(tkn) => tokens.push(tkn),
            None => { tokens.push(Token::EOF); break; },
        }
    }
    return tokens;
}

fn parse(tokens: Vec<Token>) -> Expr {
    let (ast, rest) = pcf_parser::parse_expressions(&tokens[..]);
    return
        match rest {
            [] | [Token::EOF] => ast,
            _ => Expr::Error { error: String::from("More input then expected") }
        }
}

fn interpret(ast: Expr) -> Result<Value, &'static str> {
    let visitor = InterpreterVisitor { environment: HashMap::new() };
    let value = ast.accept(&visitor);
    return
        match value {
            Some(v) => Ok(v),
            None => Err("Could not interpret program!")
        }
}

fn print_results(code: String, tokens: Vec<Token>, ast: Expr, result: Result<Value, &'static str>) {
    println!("Code: {}", code);
    println!("Tokens: {:?}", tokens);
    println!("AST: {:?}", ast);
    match result {
        Ok(v) => println!("Value: {:?}", v),
        Err(e) => println!("Error: {:?}", e)
    }
}