use std::fmt;
use std::collections::HashMap;

use crate::pcf_expression::Expr;

#[derive(Debug)]
pub enum Value {
    Num(u32),
    Bool(bool),
    Succ,
    Pred,
    IsZero,
    Closure { id: String, body: Expr, env: HashMap<String,Value> },
    Thunk { body: Expr, env: HashMap<String,Value> }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Num(n) => Value::Num(n.clone()),
            Value::Bool(b) => Value::Bool(b.clone()),
            Value::Succ => Value::Succ,
            Value::Pred => Value::Pred,
            Value::IsZero => Value::IsZero,
            Value::Closure { id, body, env } => Value::Closure { id: id.clone(), body: body.clone(), env: env.clone() },
            Value::Thunk { body, env } => Value::Thunk { body: body.clone(), env: env.clone() }
        }
    }
}