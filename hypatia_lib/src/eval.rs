use crate::{BinOp, Error, Expr, Spanned, Value};
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    fn get_var(&self, name: &str) -> Result<Value, Error> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| Error::UnknownName(name.to_string()))
    }

    fn declare_var(&mut self, name: &str, value: &Value) -> Result<Value, Error> {
        self.variables.insert(name.to_string(), value.clone());
        Ok(value.clone())
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Value {
    fn is_true(&self) -> Result<bool, Error> {
        match self {
            Value::Nothing => Ok(false),
            Value::Bool(b) => Ok(*b),
            Value::Number(n) => Err(Error::InvalidType),
        }
    }

    fn is_false(&self) -> Result<bool, Error> {
        Ok(!self.is_true()?)
    }

    fn number(&self) -> Result<f64, Error> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(Error::InvalidType),
        }
    }
}

pub fn eval_expr((expr, _): &Spanned<Expr>, env: &mut Environment) -> Result<Value, Error> {
    match &expr {
        Expr::Error => Err(Error::Parser),
        Expr::Value(value) => Ok(value.clone()),
        Expr::Variable(name) => env.get_var(name),
        Expr::Assignment(name, e) => {
            let value = eval_expr(e, env)?;
            env.declare_var(name, &value)
        }
        Expr::Call(_, _) => todo!(),
        Expr::If(cond, a, b) => {
            let cond = eval_expr(cond, env)?;
            if cond.is_true()? {
                eval_expr(a, env)
            } else {
                eval_expr(b, env)
            }
        }
        Expr::Block(expressions) => eval_block(expressions, env), // FIXME: create a new scope
        Expr::Program(expressions) => eval_block(expressions, env),
        Expr::BinOp(op, a, b) => {
            let a = eval_expr(a, env)?.number()?;
            let b = eval_expr(b, env)?.number()?;
            Ok(match op {
                crate::BinOp::Add => Value::Number(a + b),
                crate::BinOp::Div => Value::Number(a / b),
                crate::BinOp::Mul => Value::Number(a * b),
                crate::BinOp::Sub => Value::Number(a - b),
            })
        }
    }
}

fn eval_block(expressions: &Vec<Spanned<Expr>>, env: &mut Environment) -> Result<Value, Error> {
    for (i, expr) in expressions.iter().enumerate() {
        // The last expression of the block will be return value for the block expression itself
        if expressions.len() - 1 == i {
            return eval_expr(expr, env);
        }
        eval_expr(expr, env)?;
    }
    Ok(Value::Nothing)
}
