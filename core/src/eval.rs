use num::rational::Ratio;

use crate::{
    expr::{BinOp, Literal, Spanned},
    parse,
    prefixes::{PrefixName, PrefixScale, PREFIXES},
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
            Value::Quantity(q) => {
                // FIXME: We should not always normalize when displaying, still need to implement a way of
                // showing the result in the most suitable unit
                write!(f, "{}", q.clone().normalize())
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum UnitEntry {
    DeclUnit(String),
    PrefixedUnit(String),
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: Vec<HashMap<String, Value>>,
    units: HashMap<UnitEntry, Unit>,
}

impl Environment {
    pub fn new() -> Self {
        Self::without_prelude().add_prelude()
    }

    pub fn without_prelude() -> Self {
        Self {
            variables: vec![HashMap::new()],
            units: HashMap::new(),
        }
    }

    fn add_prelude(mut self) -> Self {
        let prelude_src = include_str!("prelude.hyp");
        let prelude_ast = parse(prelude_src).expect("Failed to parse prelude");
        eval(&prelude_ast, &mut self).expect("Failed to evaluate prelude");
        self
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

    fn declare_var(
        &mut self,
        name: &str,
        value: &Value,
        allow_override: bool,
    ) -> Result<(), Error> {
        // Check if this variable name is already used for a unit (which is not allowed)
        if self.get_unit(name).is_ok() && !allow_override {
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

        // Add a variable with the same name as the unit equal to a quanitity of 1 of the unit
        let quantity = Value::Quantity(Quantity(1.0, derived_unit.clone()));
        self.declare_var(long_name, &quantity, true)?;

        // add the unit
        self.units.insert(
            UnitEntry::DeclUnit(long_name.to_string()),
            derived_unit.clone(),
        );

        // Do the same if there is a short name
        if let Some(name) = short_name {
            self.units
                .insert(UnitEntry::DeclUnit(name.clone()), derived_unit.clone());
            self.declare_var(name, &quantity, true)?;
        }

        for (PrefixName(prefix_name_long, prefix_name_short), PrefixScale(base, exponent)) in
            PREFIXES.iter()
        {
            let unit = derived_unit
                .clone()
                .rescaled((*base as f64).powf(*exponent as f64));

            let quantity = Value::Quantity(Quantity(1.0, unit.clone()));

            self.units.insert(
                UnitEntry::PrefixedUnit(format!("{prefix_name_long}{long_name}")),
                unit.clone(),
            );

            self.declare_var(&format!("{prefix_name_long}{long_name}"), &quantity, true)?;

            if let Some(name) = short_name {
                self.units.insert(
                    UnitEntry::PrefixedUnit(format!("{prefix_name_short}{name}")),
                    unit,
                );

                self.declare_var(&format!("{prefix_name_short}{name}"), &quantity, true)?;
            }
        }
        Ok(())
    }

    /// Resolve the name of unit
    /// First try to get a declared unit if not found check if there is
    /// an prefixed entry.
    fn get_unit(&self, name: &str) -> Result<Unit, Error> {
        self.units
            .get(&UnitEntry::DeclUnit(name.to_string()))
            .or_else(|| self.units.get(&UnitEntry::PrefixedUnit(name.to_string())))
            .cloned()
            .ok_or_else(|| Error::UnknownName(name.to_string()))
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
            env.declare_var(name, &value, false)?;
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
