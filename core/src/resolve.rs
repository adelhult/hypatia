use crate::Error;
use std::collections::HashSet;
use syntax::expr::{Expr, Scope, Spanned};

/// Before evaluating the Expression tree we do a semantic analysis pass.
/// This allows us to find some bugs and resolve local variables to avoid scoping issues.
pub fn resolve(mut expr: Spanned<Expr>) -> Result<Spanned<Expr>, Error> {
    let mut variables = vec![HashSet::new()];
    resolve_helper(&mut expr, &mut variables)?;
    Ok(expr)
}

fn resolve_helper(
    (expr, _): &mut Spanned<Expr>,
    variables: &mut Vec<HashSet<String>>,
) -> Result<(), Error> {
    match expr {
        Expr::Error => Ok(()),
        Expr::Literal(_) => Ok(()),
        Expr::Variable(name, ref mut scope) => {
            *scope = find_scope(name, &variables);
            Ok(())
        }
        Expr::VarDeclaration(name, rhs) => {
            let current_scope = variables.last_mut().expect("No scope found");

            // You are not allowed to redeclare variables in the same scope
            if current_scope.contains(name) {
                return Err(Error::Redeclaration(name.to_string()));
            }
            current_scope.insert(name.to_string());

            resolve_helper(&mut *rhs, variables)?;
            Ok(())
        }
        Expr::VarUpdate(name, rhs, ref mut scope) => {
            resolve_helper(rhs, variables)?;

            *scope = find_scope(name, &variables);
            Ok(())
        }
        Expr::Call(func, args) => {
            resolve_helper(&mut *func, variables)?;
            for arg in args {
                resolve_helper(&mut *arg, variables)?;
            }
            Ok(())
        }
        Expr::If(cond, a, b) => {
            resolve_helper(&mut *cond, variables)?;
            resolve_helper(&mut *a, variables)?;
            resolve_helper(&mut *b, variables)
        }
        Expr::Block(expressions) => {
            variables.push(HashSet::new());
            for e in expressions {
                resolve_helper(&mut *e, variables)?;
            }
            variables.pop();
            Ok(())
        }
        Expr::Program(expressions) => {
            for e in expressions {
                resolve_helper(&mut *e, variables)?;
            }
            Ok(())
        }
        Expr::Conversion(from, to) => {
            resolve_helper(&mut *from, variables)?;
            resolve_helper(&mut *to, variables)
        }
        Expr::BinOp(_, a, b) => {
            resolve_helper(&mut *a, variables)?;
            resolve_helper(&mut *b, variables)
        }
        Expr::FunctionDecl(name, params, body) => {
            variables
                .last_mut()
                .expect("No scope found")
                .insert(name.to_string());

            variables.push(HashSet::new());
            let function_scope = variables.last_mut().unwrap();

            for param in params {
                function_scope.insert(param.to_string());
            }

            resolve_helper(&mut *body, variables)?;

            variables.pop();

            Ok(())
        }
        Expr::FunctionUpdate(name, params, body, ref mut scope) => {
            variables.push(HashSet::new());
            let function_scope = variables.last_mut().unwrap();

            for param in params {
                function_scope.insert(param.to_string());
            }

            resolve_helper(&mut *body, variables)?;

            variables.pop();

            *scope = find_scope(name, variables);

            Ok(())
        }
        Expr::BaseUnitDecl(name, short_name) => {
            let scope = variables.last_mut().expect("No scope found");
            scope.insert(name.to_string());

            if let Some(short_name) = short_name {
                scope.insert(short_name.to_string());
            }
            Ok(())
        }
        Expr::PrefixDecl(name, short_name, rhs) | Expr::DerivedUnitDecl(name, short_name, rhs) => {
            let scope = variables.last_mut().expect("No scope found");
            scope.insert(name.to_string());

            if let Some(short_name) = short_name {
                scope.insert(short_name.to_string());
            }

            resolve_helper(&mut *rhs, variables)
        }
        Expr::UnaryOp(_, operand) => resolve_helper(&mut *operand, variables),
    }
}

fn find_scope(name: &str, variables: &Vec<HashSet<String>>) -> Scope {
    // Note the resolver assumes that names it doesn't find belongs to the global scope

    for (i, scope) in variables.iter().skip(1).rev().enumerate() {
        if scope.contains(name) {
            return Scope::Local(i);
        }
    }
    return Scope::Global;
}
