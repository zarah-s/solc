use crate::mods::{
    functions::helpers::helpers::{extract_data_types_from_token, lex_to_token, print_error},
    types::types::{Mapping, MappingValue},
};

impl Mapping {
    pub fn new() -> Self {
        Self {
            key: None,
            value: None,
        }
    }

    pub fn get_return_type(&self) -> Option<&String> {
        if let Some(ref _val) = self.value {
            match _val {
                MappingValue::Mapping(_map) => _map.get_return_type(),
                MappingValue::Raw(_return) => return Some(_return),
            };
        }
        None
    }

    pub fn insert(&mut self, key: Option<String>, value: Option<MappingValue>) {
        if self.key.is_none() {
            if let Some(_key) = &key {
                if let Some(_) = extract_data_types_from_token(&lex_to_token(&_key.as_str())) {
                    self.key = key;
                } else {
                    print_error(&format!("Invalid data type \"{}\"", _key));
                }
            }
        } else if self.value.is_none() {
            if let Some(_val) = value {
                self.value = Some(_val);
            } else {
                let _key = key.clone().unwrap();
                if let Some(_) = extract_data_types_from_token(&lex_to_token(_key.as_str())) {
                    self.value = Some(MappingValue::Mapping(Box::new(Mapping {
                        key,
                        value: None,
                    })));
                } else {
                    print_error(&format!("Invalid data type \"{}\"", _key));
                }
            }
        } else {
            if let Some(ref mut node) = self.value {
                match node {
                    MappingValue::Mapping(_map) => {
                        _map.insert(key, value);
                    }
                    _ => (),
                }
            }
        }
    }
}
