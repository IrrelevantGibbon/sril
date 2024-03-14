use crate::binding_def::BindingDef;
use crate::expression::Expression;

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
}
