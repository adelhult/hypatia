use num::rational::Ratio;

use crate::{
    expr::{BinOp, Literal, NumberLiteral, Spanned, UnaryOp},
    number::Number,
    parse,
    trie::StringTrie,
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

    pub fn boolean(&self) -> Result<bool, Error> {
        if let Value::Bool(b) = self {
            Ok(*b)
        } else {
            Err(Error::InvalidType)
        }
    }

    pub fn number(&self) -> Result<Number, Error> {
        Ok(self.quantity()?.number)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nothing => write!(f, "nothing"),
            Value::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Value::Quantity(q) => {
                // FIXME: We should not always normalize when displaying, still need to implement a way of
                //  showing the result in the most suitable unit
                write!(f, "{}", q.clone().normalize())
            }
        }
    }
}

/// Used to keep track of additional information related to a Unit/Prefix
/// such as if it is a long or short name
#[derive(Debug, Clone, PartialEq)]
struct Entry<T> {
    is_long_name: bool,
    value: T,
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: Vec<HashMap<String, Value>>,
    units: HashMap<String, Entry<Unit>>,
    prefixes: StringTrie<Entry<Number>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::without_prelude().add_prelude()
    }

    pub fn without_prelude() -> Self {
        Self {
            variables: vec![HashMap::new()],
            units: HashMap::new(),
            prefixes: StringTrie::new(),
        }
    }

    fn add_prelude(mut self) -> Self {
        let prelude_src = include_str!("prelude.hyp");
        let prelude_ast = parse(prelude_src).expect("Failed to parse prelude");
        eval(&prelude_ast, &mut self).expect("Failed to evaluate prelude");
        self
    }

    fn get_var(&self, name: &str) -> Result<Value, Error> {
        // First check if the identifer is actually a unit.
        // Units used as variable will return a quantity of 1 of that unit.
        if let Ok(unit) = self.get_unit(name) {
            return Ok(Value::Quantity(Quantity {
                number: Number::one(),
                unit,
            }));
        }

        // Otherwise go through all of the scopes to find the the variable
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(name).cloned() {
                return Ok(value);
            }
        }

        // If we did not find it, the variable must be undeclare
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
            if let Value::Quantity(Quantity { number, unit }) = value {
                derived_unit = Unit(number.clone() * unit.0.clone(), unit.1.clone());
            } else {
                // The rhs must also be quantity otherwise we
                // can't derive the new unit in any sensible way
                return Err(Error::InvalidType);
            }
        } else {
            // In the case of a base unit, just make a derived unit consisting of the base unit scaled by 1
            let base_unit = BaseUnit(long_name.to_string(), short_name.clone());
            derived_unit = Unit(Number::one(), [(base_unit, Ratio::new(1, 1))].into());
        }

        // add the unit
        self.units.insert(
            long_name.to_string(),
            Entry {
                is_long_name: true,
                value: derived_unit.clone(),
            },
        );

        // Do the same if there is a short name
        if let Some(name) = short_name {
            self.units.insert(
                name.clone(),
                Entry {
                    is_long_name: false,
                    value: derived_unit,
                },
            );
        }

        Ok(())
    }

    /// Resolve the name of unit
    fn get_unit(&self, name: &str) -> Result<Unit, Error> {
        // If there is a unit with this exact name, return that
        if let Some(unit) = self.units.get(name) {
            return Ok(unit.value.clone());
        }

        // Otherwise we will check if the unit is prefixed
        for (prefix_name, prefix) in self.prefixes.search(name) {
            if let Some(unit_name) = name.strip_prefix(&prefix_name) {
                let unit = self.units.get(unit_name);

                if unit.is_none() {
                    continue;
                }

                let unit = unit.unwrap();

                // We want both the unit and prefix to be long or a short nane
                // things like "kmeter" is not accepted
                if unit.is_long_name != prefix.is_long_name {
                    continue;
                }

                return Ok(unit.value.clone().rescaled(prefix.value));
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

    fn declare_prefix(
        &mut self,
        name: &str,
        value: Number,
        is_long_name: bool,
    ) -> Result<(), Error> {
        if self.prefixes.contains_key(name) {
            Err(Error::OccupiedName(name.to_string()))
        } else {
            self.prefixes.insert(
                name,
                Entry {
                    is_long_name,
                    value,
                },
            );
            Ok(())
        }
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
        Expr::PrefixDecl(long_name, short_name, rhs) => {
            let value = eval(rhs, env)?.number()?; // FIXME: ensure that it is dimensionless
            env.declare_prefix(long_name, value.clone(), true)?;
            if let Some(name) = short_name {
                env.declare_prefix(name, value, false)?;
            }
            Ok(Value::Nothing)
        }
        Expr::UnaryOp(op, expr) => {
            let value = eval(expr, env)?;
            match op {
                UnaryOp::Negate => Ok(Value::Quantity(-value.quantity()?)),
                UnaryOp::Not => Ok(Value::Bool(!value.boolean()?)),
            }
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
        Literal::Quantity(number, name) => {
            let unit = if let Some(name) = name {
                env.get_unit(name)?
            } else {
                Unit::unitless()
            };
            Value::Quantity(Quantity {
                number: match number {
                    NumberLiteral::Binary(n) => Number::from_binary_str(n),
                    NumberLiteral::Decimal(n) => Number::from_decimal_str(n),
                    NumberLiteral::Hex(n) => Number::from_hex_str(n),
                    NumberLiteral::Scientific(base, exp, neg_sign) => Number::from_scientific_str(base, exp, *neg_sign),
                },
                unit,
            })
        }
    })
}
