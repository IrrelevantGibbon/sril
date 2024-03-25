use crate::{binding_usage::BindingUsage, block::Block, env::Env, utils, value::Value};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, number) = utils::extract_digits(s)?;
        Ok((s, Self(number.parse().unwrap())))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        utils::extract_tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::extract_tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::extract_tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::extract_tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Operator,
}

impl Operation {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, op) = Operator::new(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, rhs) = Number::new(s)?;

        Ok((s, Self { lhs, rhs, op }))
    }

    pub(crate) fn eval(&self) -> Result<Value, String> {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;

        let result = match self.op {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        };

        Ok(Value::Number(result))
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    Operation {
        lhs: Number,
        rhs: Number,
        op: Operator,
    },
    Block(Block),
    BindingUsage(BindingUsage),
}

impl Expression {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        Operation::new(s)
            .map(|(s, operation)| {
                (
                    s,
                    Self::Operation {
                        lhs: operation.lhs,
                        rhs: operation.rhs,
                        op: operation.op,
                    },
                )
            })
            .or_else(|_| Number::new(s).map(|(s, number)| (s, Self::Number(number))))
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Value, String> {
        match self {
            Self::Number(Number(n)) => Ok(Value::Number(*n)),
            Self::Operation { lhs, rhs, op } => Operation {
                lhs: *lhs,
                rhs: *rhs,
                op: *op,
            }
            .eval(),
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_usage::BindingUsage;
    use crate::env::Env;
    use crate::value::Value;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Operator::new("+"), Ok(("", Operator::Add)));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Operator::new("-"), Ok(("", Operator::Sub)));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Operator::new("*"), Ok(("", Operator::Mul)));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Operator::new("/"), Ok(("", Operator::Div)));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::new("1+2"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operator::Add
                }
            ))
        );
    }

    #[test]
    fn parse_one_plus_two_with_whitespace() {
        assert_eq!(
            Expression::new("1 + 2"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operator::Add
                }
            ))
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(10),
                rhs: Number(10),
                op: Operator::Add,
            }
            .eval(&Env::default()),
            Ok(Value::Number(20)),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Operator::Sub,
            }
            .eval(&Env::default()),
            Ok(Value::Number(-4)),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(5),
                rhs: Number(6),
                op: Operator::Mul,
            }
            .eval(&Env::default()),
            Ok(Value::Number(30)),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Operator::Div,
            }
            .eval(&Env::default()),
            Ok(Value::Number(10)),
        );
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(
            Expression::new("456"),
            Ok(("", Expression::Number(Number(456))))
        );
    }

    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.store_binding("ten".to_string(), Value::Number(10));

        assert_eq!(
            Expression::BindingUsage(BindingUsage {
                name: "ten".to_string(),
            })
            .eval(&env),
            Ok(Value::Number(10)),
        );
    }
}
