use std::fmt;

use crate::pcf_value::Value;
use crate::pcf_visitor::{Visitor, InterpreterVisitor};

#[derive(Debug)]
pub enum Expr {
    Id(String),
    Num(u32),
    Bool(bool),
    Succ,
    Pred,
    IsZero,
    Func { param: String, body: Box<Expr> },
    App { func: Box<Expr>, arg: Box<Expr> },
    Rec { func_name: String, body: Box<Expr> },
    If { cond: Box<Expr>, t_val: Box<Expr>, f_val: Box<Expr> },
    Error { error: String },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Expr::Id(s) => Expr::Id(s.clone()),
            Expr::Num(n) => Expr::Num(n.clone()),
            Expr::Bool(b) => Expr::Bool(b.clone()),
            Expr::Succ => Expr::Succ,
            Expr::Pred => Expr::Pred,
            Expr::IsZero => Expr::IsZero,
            Expr::Func { param, body } => Expr::Func { param: param.clone(), body: body.clone() },
            Expr::App { func, arg } => Expr::App { func: func.clone(), arg: arg.clone() },
            Expr::Rec { func_name, body } => Expr::Rec { func_name: func_name.clone(), body: body.clone() },
            Expr::If { cond, t_val, f_val } => Expr::If { cond: cond.clone(), t_val: t_val.clone(), f_val: f_val.clone() },
            Expr::Error { error } => Expr::Error { error: error.clone() },
        }
    }
}

impl Expr {
    pub fn accept(&self, v: &InterpreterVisitor) -> Option<Value> {
        match self {
            Expr::Id(s) => {
                v.visit_id(s.clone())
            },
            Expr::Num(n) => {
                v.visit_num(n.clone())
            },
            Expr::Bool(b) => {
                v.visit_bool(b.clone())
            },
            Expr::Succ => {
                v.visit_succ()
            },
            Expr::Pred => {
                v.visit_pred()
            },
            Expr::IsZero => {
                v.visit_is_zero()
            },
            Expr::Func { param, body} => {
                v.visit_func(param.clone(), *body.clone())
            },
            Expr::App { func, arg} => {
                v.visit_app(*func.clone(), *arg.clone())
            },
            Expr::Rec { func_name, body} => {
                v.visit_rec(func_name.clone(), *body.clone())
            },
            Expr::If { cond, t_val, f_val} => {
                v.visit_if(*cond.clone(), *t_val.clone(), *f_val.clone())
            },
            Expr::Error { error: _e} => {
                None
            }
        }
    }
}