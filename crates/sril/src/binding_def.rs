use crate::env::Env;
use crate::expression::Expression;
use crate::utils;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Let,
    Const,
}

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub val: Expression,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::extract_tag("let", s)?;
        let (s, _) = utils::extract_required_whitespaces(s)?;

        let (s, name) = utils::extract_identifier(s)?;
        let (s, _) = utils::extract_required_whitespaces(s)?;

        let s = utils::extract_tag("=", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, val) = Expression::new(s)?;
        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Value, String> {
        env.store_binding(self.name.clone(), self.val.eval(env)?);
        Ok(Value::Unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{Number, Operator};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expression::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Operator::Div,
                    },
                },
            ),)
        );
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(
            BindingDef::new("letaaa=1+2"),
            Err("Expected space".to_string()),
        );
    }

    // #[test]
    // fn eval_number_expression() {
    //     let mut env = Env::default();
    //     let (_, bd) = BindingDef::new("let a = 10 / 2");
    //     bd.eval(&mut env);
    //     assert_eq!(env.retrieve_bindings(bd.name), &Value::Number(5))
    // }
}
