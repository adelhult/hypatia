use num::rational::Ratio;

use crate::{
    expr::{BinOp, Literal, Spanned},
    units::{BaseUnit, Quantity, Unit},
    Error, Expr,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nothing,
    Bool(bool),
    Quantity(Quantity),
}

impl Value {
    pub fn is_true(&self) -> Result<bool, Error> {
        match self {
            Value::Nothing => Ok(false),
            Value::Bool(b) => Ok(*b),
            Value::Quantity(_) => Err(Error::InvalidType),
        }
    }

    pub fn is_false(&self) -> Result<bool, Error> {
        Ok(!self.is_true()?)
    }

    pub fn quantity(&self) -> Result<Quantity, Error> {
        if let Value::Quantity(q) = self {
            Ok(q.clone())
        } else {
            Err(Error::InvalidType)
        }
    }

    pub fn number(&self) -> Result<f64, Error> {
        Ok(self.quantity()?.0)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nothing => write!(f, "nothing"),
            Value::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::Quantity(q) => write!(f, "{q}"),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    variables: Vec<HashMap<String, Value>>,
    units: Vec<HashMap<String, Unit>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: vec![HashMap::new()],
            units: vec![HashMap::new()],
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
        // Check if this variable name is already used for a unit (which is not allowed)
        if self.get_unit(name).is_ok() {
            return Err(Error::OccupiedName(name.to_string()));
        }

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
        // Check if this variable name is already used for a unit (which is not allowed)
        if self.get_unit(name).is_ok() {
            return Err(Error::OccupiedName(name.to_string()));
        }

        self.variables
            .last_mut()
            .expect("No scope exists")
            .insert(name.to_string(), value.clone());
        Ok(())
    }

    fn declare_unit(
        &mut self,
        long_name: &str,
        short_name: &Option<String>,
        derivation: Option<&Value>,
    ) -> Result<(), Error> {
        let derived_unit;

        // handle derived units
        // unit mile mi = 1 609.344 m
        if let Some(value) = derivation {
            if let Value::Quantity(Quantity(mag, unit)) = value {
                derived_unit = Unit(mag * unit.0, unit.1.clone());
            } else {
                // The rhs must also be quantity otherwise we
                // can't derive the new unit in any sensible way
                return Err(Error::InvalidType);
            }
        } else {
            // In the case of a base unit, just make a derived unit consisting of the base unit scaled by 1
            let base_unit = BaseUnit(long_name.to_string(), short_name.clone());
            derived_unit = Unit(1.0, [(base_unit, Ratio::new(1, 1))].into());
        }

        let current_scope = self.units.last_mut().expect("No scope exists");

        if let Some(name) = short_name {
            current_scope.insert(name.clone(), derived_unit.clone());
        }

        current_scope.insert(long_name.to_string(), derived_unit);

        Ok(())
    }

    fn get_unit(&self, name: &str) -> Result<Unit, Error> {
        for scope in self.units.iter().rev() {
            if let Some(value) = scope.get(name).cloned() {
                return Ok(value);
            }
        }
        Err(Error::UnknownName(name.to_string()))
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
        Expr::Literal(literal) => eval_literal(literal, env),
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
            let a = eval(a, env)?.quantity()?;
            let b = eval(b, env)?.quantity()?;
            Ok(Value::Quantity(match op {
                BinOp::Add => (a + b)?,
                BinOp::Sub => (a - b)?,
                BinOp::Div => a / b,
                BinOp::Mul => a * b,
            }))
        }
        Expr::BaseUnitDecl(long_name, short_name) => {
            env.declare_unit(long_name, short_name, None)?;
            Ok(Value::Nothing)
        }
        Expr::DerivedUnitDecl(long_name, short_name, expr) => {
            // FIXME: Maybe disallow "normal" variables to be used in the rhs
            let value = eval(expr, env)?;
            env.declare_unit(long_name, short_name, Some(&value))?;
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

fn eval_literal(literal: &Literal, env: &mut Environment) -> Result<Value, Error> {
    Ok(match literal {
        Literal::Nothing => Value::Nothing,
        Literal::Bool(b) => Value::Bool(*b),
        Literal::Quantity(magnitude, name) => {
            let unit = if let Some(name) = name {
                env.get_unit(name)?
            } else {
                Unit::unitless()
            };
            Value::Quantity(Quantity(*magnitude, unit))
        }
    })
}
