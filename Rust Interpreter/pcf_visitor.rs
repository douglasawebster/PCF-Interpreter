use std::collections::HashMap;

use crate::pcf_expression::Expr;
use crate::pcf_value::Value;

pub trait Visitor<T> {
    fn visit_id(&self, id: String) -> T;
    fn visit_num(&self, n: u32) -> T;
    fn visit_bool(&self, b: bool) -> T;
    fn visit_succ(&self) -> T;
    fn visit_pred(&self) -> T;
    fn visit_is_zero(&self) -> T;
    fn visit_func(&self, param: String, body: Expr) -> T;
    fn visit_app(&self, func: Expr, arg: Expr) -> T;
    fn visit_rec(&self, func_name: String, body: Expr) -> T;
    fn visit_if(&self, cond: Expr, t_val: Expr, f_val: Expr) -> T;
}

pub struct InterpreterVisitor {
    pub environment: HashMap<String,Value>
}

impl Visitor<Option<Value>> for InterpreterVisitor {
    fn visit_id(&self, id: String) -> Option<Value> {
        let id_val = self.environment.get(&id)?;
        match id_val {
            Value::Thunk { body, env } => {
                let new_visitor = InterpreterVisitor { environment: env.clone() };
                body.accept(&new_visitor)
            }
            _ => Some(id_val.clone())
        }
    }

    fn visit_num(&self, n: u32) -> Option<Value> {
        Some(Value::Num(n))
    }

    fn visit_bool(&self, b: bool) -> Option<Value> {
        Some(Value::Bool(b))
    }

    fn visit_succ(&self) -> Option<Value> {
        Some(Value::Succ)
    }

    fn visit_pred(&self) -> Option<Value> {
        Some(Value::Pred)
    }

    fn visit_is_zero(&self) -> Option<Value> {
        Some(Value::IsZero)
    }

    fn visit_func(&self, param: String, body: Expr) -> Option<Value> {
        Some(Value::Closure { id: param, body: body, env: self.environment.clone() })
    }

    fn visit_app(&self, func: Expr, arg: Expr) -> Option<Value> {
        let eval_func = func.accept(self)?;
        let eval_arg = arg.accept(self)?;

        match (eval_func, eval_arg) {
            (Value::Succ, Value::Num(n)) => Some(Value::Num(n+1)),
            (Value::Pred, Value::Num(n)) => if n > 0 { Some(Value::Num(n-1)) } else { Some(Value::Num(0)) }
            (Value::IsZero, Value::Num(n)) => if n == 0 { Some(Value::Bool(true)) } else { Some(Value::Bool(false)) },
            (Value::Closure { id, body, env }, arg) => {
                let mut new_env = env.clone();
                new_env.insert(id, arg);
                let new_visitor = InterpreterVisitor { environment: new_env };
                body.accept(&new_visitor)
            }
            _ => None
        }
    }

    fn visit_rec(&self, func_name: String, body: Expr) -> Option<Value> {
        let value = Value::Thunk { body: Expr::Rec { func_name: func_name.clone(), body: Box::new(body.clone()) }, env: self.environment.clone() };
        let mut new_env = self.environment.clone();
        new_env.insert(func_name.clone(), value);
        let new_visitor = InterpreterVisitor { environment: new_env };
        body.accept(&new_visitor)
    }

    fn visit_if(&self, cond: Expr, t_val: Expr, f_val: Expr) -> Option<Value> {
        let bool_val = cond.accept(self)?;
        match bool_val {
            Value::Bool(b) => if b { t_val.accept(self) } else { f_val.accept(self) },
            _ => None
        }
    }
}
