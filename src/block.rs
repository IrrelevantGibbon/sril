use crate::{env::Env, statement::Statement, utils, value::Value};

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

    pub(crate) fn eval(&self, env: &Env) -> Result<Value, String> {
        if self.statements.is_empty() {
            return Ok(Value::Unit);
        }

        let mut env = Env::default();

        let statements_except_last = &self.statements[..self.statements.len() - 1];

        for statement in statements_except_last {
            statement.eval(&mut env)?;
        }

        self.statements.last().unwrap().eval(&mut env)
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

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block {
                statements: Vec::new()
            }
            .eval(&Env::default()),
            Ok(Value::Unit),
        );
    }

    #[test]
    fn eval_block_with_one_expr() {
        assert_eq!(
            Block {
                statements: vec![Statement::Expression(Expression::Number(Number(25)))],
            }
            .eval(&Env::default()),
            Ok(Value::Number(25)),
        );
    }

    #[test]
    fn eval_block_with_binding_def_and_usage() {
        assert_eq!(
            Block {
                statements: vec![
                    Statement::BindingDef(BindingDef {
                        name: "one".to_string(),
                        val: Expression::Number(Number(1)),
                    }),
                    Statement::Expression(Expression::BindingUsage(BindingUsage {
                        name: "one".to_string(),
                    })),
                ],
            }
            .eval(&Env::default()),
            Ok(Value::Number(1)),
        );
    }
}
