use crate::units::BaseUnit;
use crate::{
    expr::{BinOp, Spanned},
    Error, Expr, Value,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    variables: Vec<HashMap<String, Value>>,
    base_units: Vec<Vec<BaseUnit>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: vec![HashMap::new()],
            base_units: vec![vec![BaseUnit("Unitless".to_string(), Some("".to_string()))]],
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

    fn update_var(&mut self, name: &str, value: &Value) -> Result<(), Error> {
        for scope in self.variables.iter_mut().rev() {
            if !scope.contains_key(name) {
                continue;
            }
            scope.insert(name.to_string(), value.clone());
            return Ok(());
        }
        Err(Error::UpdateNonExistentVar(name.to_string()))
    }

    fn declare_var(&mut self, name: &str, value: &Value) -> Result<(), Error> {
        self.variables
            .last_mut()
            .expect("No scope exists")
            .insert(name.to_string(), value.clone());
        Ok(())
    }

    fn declare_base_unit(
        &mut self,
        long_name: &str,
        short_name: &Option<String>,
    ) -> Result<(), Error> {
        self.base_units
            .last_mut()
            .expect("No scope exists")
            .push(BaseUnit(long_name.to_string(), short_name.clone()));
        Ok(())
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

/// Evaluate an AST of Expr nodes into a Value
pub fn eval((expr, _): &Spanned<Expr>, env: &mut Environment) -> Result<Value, Error> {
    match &expr {
        Expr::Error => Err(Error::ErrorNode),
        Expr::Value(value) => Ok(value.clone()),
        Expr::Variable(name) => env.get_var(name),
        Expr::VarDeclaration(name, rhs) => {
            let value = eval(rhs, env)?;
            env.declare_var(name, &value)?;
            Ok(value)
        }
        Expr::VarUpdate(name, rhs) => {
            let value = eval(rhs, env)?;
            env.update_var(name, &value)?;
            Ok(value)
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
        Expr::BaseUnitDeclaration(long_name, short_name) => {
            env.declare_base_unit(long_name, short_name)?;
            Ok(Value::Nothing)
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
