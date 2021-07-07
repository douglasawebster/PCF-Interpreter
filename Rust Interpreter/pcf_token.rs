use std::fmt;

#[derive(Debug)]
pub enum Token {
    Id(String),
    Num(u32),
    IfSmy,
    ThenSym,
    ElseSym,
    TrueSym,
    FalseSym,
    SuccSym,
    PredSym,
    IsZeroSym,
    FnSym,
    RecSym,
    Equal,
    FnArrow,
    LParen,
    RParen,
    LetSym,
    InSym,
    EndSym,
    EOF,
    Error(String)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Id(s) => Token::Id(s.clone()),
            Token::Num(n) => Token::Num(n.clone()),
            Token::IfSmy        => Token::IfSmy,
            Token::ThenSym      => Token::ThenSym,
            Token::ElseSym      => Token::ElseSym,
            Token::TrueSym      => Token::TrueSym,
            Token::FalseSym     => Token::FalseSym,
            Token::SuccSym      => Token::SuccSym,
            Token::PredSym      => Token::PredSym,
            Token::IsZeroSym    => Token::IsZeroSym,
            Token::FnSym        => Token::FnSym,
            Token::RecSym       => Token::RecSym,
            Token::Equal        => Token::Equal,
            Token::FnArrow      => Token::FnArrow,
            Token::LParen       => Token::LParen,
            Token::RParen       => Token::RParen,
            Token::LetSym       => Token::LetSym,
            Token::InSym        => Token::InSym,
            Token::EndSym       => Token::EndSym,
            Token::EOF          => Token::EOF,
            Token::Error(s) => Token::Error(s.clone())
        }
    }
}