use crate::binding_def::BindingDef;
use crate::env::Env;
use crate::expression::Expression;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Statement {
    BindingDef(BindingDef),
    Expression(Expression),
}

impl Statement {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_definition)| (s, Self::BindingDef(binding_definition)))
            .or_else(|_| {
                Expression::new(s).map(|(s, expression)| (s, Self::Expression(expression)))
            })
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Value, String> {
        match self {
            Statement::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Value::Unit)
            }
            Statement::Expression(expression) => expression.eval(&env),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::expression::{Number, Operator};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new("let a = 10"),
            Ok((
                "",
                Statement::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expression::Number(Number(10)),
                }),
            )),
        );
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Statement::new("1+1"),
            Ok((
                "",
                Statement::Expression(Expression::Operation {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Operator::Add,
                }),
            )),
        );
    }

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Statement::BindingDef(BindingDef {
                name: "whatever".to_string(),
                val: Expression::Number(Number(-10)),
            })
            .eval(&mut Env::default()),
            Ok(Value::Unit),
        );
    }

    #[test]
    fn eval_expr() {
        assert_eq!(
            Statement::Expression(Expression::Number(Number(5))).eval(&mut Env::default()),
            Ok(Value::Number(5)),
        );
    }
}
