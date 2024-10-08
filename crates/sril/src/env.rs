use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    bindings: HashMap<String, Value>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }

    pub(crate) fn store_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Value, String> {
        self.get_binding_value_without_error_msg(name)
            .ok_or_else(|| format!("binding with name ‘{}’ does not exist", name))
    }

    pub fn get_binding_value_without_error_msg(&self, name: &str) -> Option<Value> {
        self.bindings.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| parent.get_binding_value_without_error_msg(name))
        })
    }
}
