use crate::mods::types::types::Conditionals;

impl Conditionals {
    pub fn new(condition: Vec<crate::mods::types::types::Token>) -> Self {
        Self {
            condition,
            arm: Vec::new(),
            el: None,
            elif: Vec::new(),
        }
    }
}
