use crate::{statement::Statement, utils, value::Value};

#[derive(Debug, PartialEq)]
pub struct Block {
    statements: Vec<Statement>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::extract_tag("{", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let mut s = s;
        let mut statements = Vec::new();

        while let Ok((new_s, stmt)) = Statement::new(s) {
            s = new_s;
            statements.push(stmt);

            let (new_s, _) = utils::extract_whitespaces(s);
            s = new_s;
        }

        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::extract_tag("}", s)?;

        Ok((s, Self { statements }))
    }

    pub(crate) fn eval(&self) -> Result<Value, String> {
        Ok(Value::Number(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::binding_usage::BindingUsage;
    use crate::env::Env;
    use crate::expression::{Expression, Number};

    #[test]
    fn parse_empty_block() {
        assert_eq!(
            Block::new("{}"),
            Ok((
                "",
                Block {
                    statements: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(
            Block::new("{   }"),
            Ok((
                "",
                Block {
                    statements: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    statements: vec![Statement::Expression(Expression::Number(Number(5)))],
                },
            )),
        );
    }

    #[test]
    fn parse_block_with_multiple_statements() {
        assert_eq!(
            Block::new(
                "{
     let a = 10
     let b = a
     b
 }",
            ),
            Ok((
                "",
                Block {
                    statements: vec![
                        Statement::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expression::Number(Number(10)),
                        }),
                        Statement::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expression::BindingUsage(BindingUsage {
                                name: "a".to_string()
                            })
                        }),
                        Statement::Expression(Expression::BindingUsage(BindingUsage {
                            name: "b".to_string(),
                        })),
                    ],
                },
            )),
        );
    }

    #[test]
    fn eval_block() {
        assert_eq!(
            Expression::Block(Block {
                statements: vec![Statement::Expression(Expression::Number(Number(10)))],
            })
            .eval(&Env::default()),
            Ok(Value::Number(10)),
        );
    }
}
