use crate::env::Env;
use crate::utils;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
    pub name: String,
}

impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_identifier(s)?;
        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Value, String> {
        env.get_binding_value(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_usage() {
        assert_eq!(
            BindingUsage::new("a"),
            Ok((
                "",
                BindingUsage {
                    name: "a".to_string()
                }
            ))
        )
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Value::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .eval(&mut env),
            Ok(Value::Number(10)),
        );
    }

    #[test]
    fn eval_non_existent_binding_usage() {
        let mut empty_env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "i_dont_exist".to_string(),
            }
            .eval(&mut empty_env),
            Err("binding with name ‘i_dont_exist’ does not exist".to_string()),
        );
    }
}
