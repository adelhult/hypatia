use num::rational::Ratio;
use std::sync::{Arc, Mutex};

use crate::{
    expr::{BinOp, Literal, NumberLiteral, Spanned, UnaryOp},
    number::Number,
    parse,
    trie::StringTrie,
    units::{BaseUnit, Quantity, Unit},
    Error, Expr,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    Nothing,
    Bool(bool),
    Quantity(Quantity),
    Function(Function),
}

impl Value {
    pub fn is_true(&self) -> Result<bool, Error> {
        match self {
            Value::Nothing => Ok(false),
            Value::Bool(b) => Ok(*b),
            Value::Quantity(_) => Err(Error::InvalidType),
            Value::Function(_) => Err(Error::InvalidType),
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
            Value::Function(_) => write!(f, "Function"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    body: Spanned<Expr>,
    parameters: Vec<String>,
    env: Environment, // FIXME: I need to make the Environment a lot cheaper to clone, should just be a smart pointer.
                      // That means that I need to move the units and prefixes into Arc<Mutex<..>>
}

/// Used to keep track of additional information related to a Unit/Prefix
/// such as if it is a long or short name
#[derive(Debug, Clone, PartialEq)]
struct Entry<T> {
    is_long_name: bool,
    value: T,
}

#[derive(Debug, Clone)]
struct VariableScope {
    table: HashMap<String, Value>,
    // Note: Will need to be thread safe since the Environment
    // is stored in a global variable in implementation the front-end
    outer: Option<Arc<Mutex<Self>>>,
}

impl VariableScope {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
            outer: None,
        }
    }

    fn get_var(&self, name: &str) -> Option<Value> {
        self.table.get(name).cloned().or_else(|| {
            self.outer
                .as_ref()
                .and_then(|outer| outer.lock().unwrap().get_var(name))
        })
    }

    fn declare_var(&mut self, name: &str, value: Value) {
        self.table.insert(name.to_string(), value);
    }

    fn update_var(&mut self, name: &str, value: Value) -> Result<(), Error> {
        if self.table.contains_key(name) {
            self.table.insert(name.to_string(), value);
            Ok(())
        } else if let Some(outer) = self.outer.as_ref() {
            outer.lock().unwrap().update_var(name, value)
        } else {
            Err(Error::UpdateNonExistentVar(name.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: Arc<Mutex<VariableScope>>,
    units: Arc<Mutex<HashMap<String, Entry<Unit>>>>,
    unit_names:
        Arc<Mutex<HashMap<BTreeMap<BaseUnit, Ratio<i32>>, HashSet<(String, Option<String>)>>>>,
    prefixes: Arc<Mutex<StringTrie<Entry<Number>>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::without_prelude().add_prelude()
    }

    pub fn without_prelude() -> Self {
        Self {
            variables: Arc::new(Mutex::new(VariableScope::new())),
            units: Arc::new(Mutex::new(HashMap::new())),
            unit_names: Arc::new(Mutex::new(HashMap::new())),
            prefixes: Arc::new(Mutex::new(StringTrie::new())),
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
        self.variables
            .lock()
            .unwrap()
            .get_var(name)
            .ok_or_else(|| Error::UnknownName(name.to_string()))
    }

    fn update_var(&mut self, name: &str, value: &Value) -> Result<(), Error> {
        // Check if this variable name is already used for a unit (which is not allowed)
        if self.get_unit(name).is_ok() {
            return Err(Error::OccupiedName(name.to_string()));
        }

        self.variables
            .lock()
            .unwrap()
            .update_var(name, value.clone())
    }

    fn declare_var(&mut self, name: &str, value: &Value) -> Result<(), Error> {
        // Check if this variable name is already used for a unit (which is not allowed)
        if self.get_unit(name).is_ok() {
            return Err(Error::OccupiedName(name.to_string()));
        }

        self.variables
            .lock()
            .unwrap()
            .declare_var(name, value.clone());
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

        let mut units = self.units.lock().unwrap();
        // add the unit
        units.insert(
            long_name.to_string(),
            Entry {
                is_long_name: true,
                value: derived_unit.clone(),
            },
        );

        // Do the same if there is a short name
        if let Some(name) = short_name {
            units.insert(
                name.clone(),
                Entry {
                    is_long_name: false,
                    value: derived_unit.clone(),
                },
            );
        }

        // Also, create a entry with the base unit set that maps to the name so we can make
        // cheap lookups later when we want to display a nice name of a unit.
        // For example, [kg^1, m^1, s^-2] -> ("Newton", "N").
        let mut unit_names = self.unit_names.lock().unwrap();
        let entry = (long_name.to_string(), short_name.clone());
        if let Some(names) = unit_names.get_mut(&derived_unit.1) {
            names.insert(entry);
        } else {
            unit_names.insert(derived_unit.1, [entry].into());
        }

        Ok(())
    }

    /// Resolve the name of unit
    fn get_unit(&self, name: &str) -> Result<Unit, Error> {
        let units = self.units.lock().unwrap();
        let prefixes = self.prefixes.lock().unwrap();

        // If there is a unit with this exact name, return that
        if let Some(unit) = units.get(name) {
            return Ok(unit.value.clone());
        }

        // Otherwise we will check if the unit is prefixed

        for (prefix_name, prefix) in prefixes.search(name) {
            if let Some(unit_name) = name.strip_prefix(&prefix_name) {
                let Some(unit) = units.get(unit_name) else {
                    continue;
                };

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

    fn get_unit_names(
        &self,
        base_units: &BTreeMap<BaseUnit, Ratio<i32>>,
    ) -> HashSet<(String, Option<String>)> {
        let unit_names = self.unit_names.lock().unwrap();
        unit_names
            .get(base_units)
            .cloned()
            .unwrap_or_else(|| HashSet::new())
    }

    fn push_scope(&mut self) {
        let outer_scope = Arc::clone(&self.variables);
        let new_scope = VariableScope {
            outer: Some(outer_scope),
            table: HashMap::new(),
        };

        self.variables = Arc::new(Mutex::new(new_scope));
    }

    fn pop_scope(&mut self) {
        let outer_scope = match &self.variables.lock().unwrap().outer {
            Some(outer_scope) => Arc::clone(outer_scope),
            None => panic!("No outer scope"),
        };
        self.variables = outer_scope;
    }

    fn declare_prefix(
        &mut self,
        name: &str,
        value: Number,
        is_long_name: bool,
    ) -> Result<(), Error> {
        let mut prefixes = self.prefixes.lock().unwrap();

        if prefixes.contains_key(name) {
            Err(Error::OccupiedName(name.to_string()))
        } else {
            prefixes.insert(
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
        Expr::Call(callable, arguments) => {
            let Value::Function(mut function) = eval(callable, env)? else {
               return Err(Error::InvalidType);
            };

            if function.parameters.len() != arguments.len() {
                return Err(Error::InvalidType);
            }

            // Create a new scope and add all the arguments
            function.env.push_scope();
            // Evaluate  the arguments (note: use the env at the call site)
            let values: Vec<Result<_, _>> = arguments.iter().map(|arg| eval(arg, env)).collect();

            for (name, value) in function.parameters.iter().zip(values.into_iter()) {
                env.declare_var(name, &value?)?;
            }

            // Finally, evaluate the function body
            // (note: important to use the environment from the actual closure here)
            eval(&function.body, &mut function.env)
        }

        Expr::FunctionDecl(name, parameters, body) => {
            let function = Value::Function(Function {
                parameters: parameters.clone(),
                body: *body.clone(),
                env: env.clone(),
            });

            env.declare_var(name, &function)?;

            Ok(function)
        }
        Expr::FunctionUpdate(name, parameters, body) => {
            let function = Value::Function(Function {
                parameters: parameters.clone(),
                body: *body.clone(),
                env: env.clone(),
            });

            env.update_var(name, &function)?;

            Ok(function)
        }
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

/// Given a Quantity get the best matching unit to display the quantity as.
/// Returns a new quantity which might be rescaled if there is no perfect match and
/// long and short name of the unit.
pub fn format_unit(
    quantity: Quantity,
    env: &Environment,
    ) -> (Quantity, (String, Option<String>)) {
    let Quantity { number, unit } = &quantity;
    let Unit(scale, base_units) = unit;

    let matches = env.get_unit_names(&base_units);

    // Compare the scale of this unit with the scale used in our Quantity, we pick
    // the unit with the lowest difference in scale as our prefered unit.
    // TODO: Rewrite this. This code ended up super imperative and kinda ugly. It was
    // hard to use util methods like min_by_key on Number since it lacks the Ord trait.
    let mut best_diff = None;
    let mut best_match = None;

    for (long_name, short_name) in matches {
        let Ok(Unit(other_scale, _)) = env.get_unit(&long_name) else {
            continue;
        };

        // Compare the scale of the unit found in our quantity with
        // all possible matches
        let diff = Number::abs(other_scale - scale.clone());

        // Save the one with the smallest difference so far.
        if let Some(best_diff_value) = best_diff.clone() {
            if diff < best_diff_value {
                best_diff = Some(diff.clone());
                best_match = Some((long_name, short_name));
            }
        } else {
            best_diff = Some(diff.clone());
            best_match = Some((long_name, short_name));
        }

        if diff == Number::zero() {
            break;
        }
    }

    let Some((ref long_name, _)) = best_match else {
        // If we did not find a matching named unit, just rescale the quantity and present it in base units
        // For example, instead of Quantity(2, Unit(1337, meter * second))
        //                      -> Quantity(2 * 1337, Unit( 1, meter * second)
        //                      -> "2674000  m * s"
        let rescaled_unit = unit.clone().rescaled(Number::one() / scale.clone());
        let rescaled_quantity = Quantity {number: number.clone() * scale.clone(), unit: rescaled_unit.clone()};
        return (
                rescaled_quantity,
            (format!("{rescaled_unit}"), None)
        );
    };

    // If we instead found a match. Then the best matching unit will be our target, so, let's
    // rescale the provided quantity to fit the unit
    let Unit(target_scale, _) = env.get_unit(&long_name).unwrap();

    // Now, we might need to rescale the original quantity to fit we the unit
    // that we have selected.
    let rescaled_quantity = Quantity {
        number: number.clone() * scale.clone() / target_scale.clone(),
        unit: Unit(target_scale, base_units.clone()),
    };

    (rescaled_quantity, best_match.unwrap())
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
                    NumberLiteral::Scientific(base, exp, neg_sign) => {
                        Number::from_scientific_str(base, exp, *neg_sign)
                    }
                },
                unit,
            })
        }
    })
}
