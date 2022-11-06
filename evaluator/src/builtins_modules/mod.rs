use std::collections::HashMap;

use super::object::Object;

pub mod math;

pub fn new_builtins() -> HashMap<String, HashMap<String, Object>> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("frog::math"), math::new());
    builtins
}
