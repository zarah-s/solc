use regex::Regex;

use crate::mods::{
    constants::constants::{DATA_TYPES, KEYWORDS, SYMBOLS},
    functions::helpers::helpers::{detokenize, lex_to_token},
    types::types::{LineDescriptions, Token},
};

impl LineDescriptions {
    pub fn to_string(self) -> String {
        format!("{}&=>{}%%", self.text, self.line)
    }

    pub fn to_struct(value: String) -> Vec<Self> {
        let splited: Vec<&str> = value.split("%%").collect();
        let mut return_value: Vec<Self> = Vec::new();
        for split in splited {
            let val: Vec<&str> = split.split("&=>").collect();
            if !val.first().unwrap().trim().is_empty() {
                return_value.push(Self {
                    text: val.first().unwrap().to_string(),
                    line: val.last().unwrap().parse().unwrap(),
                })
            }
        }

        return_value
    }

    pub fn from_token_to_string(token: &Token) -> String {
        return detokenize(&token);
    }

    pub fn to_token(input: &str) -> Vec<Token> {
        let mut lex: Vec<String> = Vec::new();
        let mut combined_char = String::new();
        let mut lexems: Vec<Token> = Vec::new();
        let mut opened_quotations = 0;
        let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
        for (index, character) in input.chars().enumerate() {
            if character == '"' || character == '\'' {
                if opened_quotations == 0 {
                    opened_quotations += 1;
                } else {
                    opened_quotations = 0
                }
            }
            if character.is_whitespace() && !combined_char.trim().is_empty() {
                if opened_quotations > 0 {
                    combined_char.push(character)
                } else {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
            } else if let Some(_) = SYMBOLS
                .iter()
                .find(|pred| pred == &&character.to_string().as_str())
            {
                if !combined_char.trim().is_empty() {
                    if opened_quotations > 0 {
                        combined_char.push(character)
                    } else {
                        lex.push(combined_char.trim().to_string());
                        combined_char.clear();
                    }
                }
                lex.push(character.to_string());
            } else if let Some(_) = [KEYWORDS.to_vec(), DATA_TYPES.to_vec(), SYMBOLS.to_vec()]
                .concat()
                .iter()
                .find(|pred| pred == &&combined_char.as_str().trim())
            {
                let mut contains = false;
                for data_type in DATA_TYPES {
                    if !data_type.contains(&format!("{}{}", combined_char, character).as_str()) {
                        contains = true;
                        break;
                    }
                }

                if contains {
                    combined_char.push(character);
                    continue;
                }

                if let Some(_) = identifier_regex.find(character.to_string().as_str()) {
                    combined_char.push(character);
                } else {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
            } else {
                combined_char.push_str(character.to_string().as_str());
                if index == input.len() - 1 {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
            }
        }
        for lexed in lex {
            lexems.push(lex_to_token(&lexed));
        }

        // panic!("{:#?}", lexems);
        lexems
    }
}
