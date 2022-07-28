use crate::{
    parser::{BinOp, Spanned},
    Error, Expr, Value,
};
use std::collections::HashMap;

pub struct Environment {
    variables: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: vec![HashMap::new()],
        }
    }

    fn get_var(&self, name: &str) -> Result<Value, Error> {
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(name).cloned() {
                return Ok(value);
            }
        }
        Err(Error::UnknownName(name.to_string()))
    }

    fn declare_var(&mut self, name: &str, value: &Value) -> Result<Value, Error> {
        self.variables
            .last_mut()
            .expect("No scope exists")
            .insert(name.to_string(), value.clone());
        Ok(value.clone())
    }

    fn push_scope(&mut self) {
        self.variables.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.variables.pop();
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
            Value::Number(_) => Err(Error::InvalidType),
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

/// Evaluate an AST of Expr nodes into a Value
pub fn eval((expr, _): &Spanned<Expr>, env: &mut Environment) -> Result<Value, Error> {
    match &expr {
        Expr::Error => Err(Error::ErrorNode),
        Expr::Value(value) => Ok(value.clone()),
        Expr::Variable(name) => env.get_var(name),
        Expr::Assignment(name, e) => {
            let value = eval(e, env)?;
            env.declare_var(name, &value)
        }
        Expr::Call(_, _) => todo!(),
        Expr::If(cond, a, b) => {
            let cond = eval(cond, env)?;
            if cond.is_true()? {
                eval(a, env)
            } else {
                eval(b, env)
            }
        }
        Expr::Block(expressions) => {
            env.push_scope();
            let block_result = eval_block(expressions, env);
            env.pop_scope();
            block_result
        }
        Expr::Program(expressions) => eval_block(expressions, env),
        Expr::BinOp(op, a, b) => {
            let a = eval(a, env)?.number()?;
            let b = eval(b, env)?.number()?;
            Ok(match op {
                BinOp::Add => Value::Number(a + b),
                BinOp::Div => Value::Number(a / b),
                BinOp::Mul => Value::Number(a * b),
                BinOp::Sub => Value::Number(a - b),
            })
        }
    }
}

fn eval_block(expressions: &Vec<Spanned<Expr>>, env: &mut Environment) -> Result<Value, Error> {
    for (i, expr) in expressions.iter().enumerate() {
        // The last expression of the block will be return value for the block expression itself
        if expressions.len() - 1 == i {
            return eval(expr, env);
        }
        eval(expr, env)?;
    }
    Ok(Value::Nothing)
}
