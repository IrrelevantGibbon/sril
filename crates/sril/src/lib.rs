pub mod binding_def;
pub mod binding_usage;
pub mod block;
pub mod expression;
pub mod statement;
mod utils;

pub mod env;
pub mod value;

#[derive(Debug)]
pub struct Parse(statement::Statement);

impl Parse {
    pub fn eval(&self, env: &mut env::Env) -> Result<value::Value, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = statement::Statement::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}
