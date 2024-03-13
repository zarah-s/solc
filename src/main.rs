use regex::Regex;
use std::{
    env, fs, process,
    time::{self, SystemTime},
};

mod mods;

use mods::{
    constants::constants::{DATA_TYPES, KEYWORDS, SYMBOLS},
    functions::{
        controllers::{
            process_enum::extract_enum, process_file_contents::process_file_contents,
            process_function::extract_functions, process_state_variables::extract_global_variables,
            process_struct::extract_struct, strip_comments::strip_comments,
            structure_to_line_descriptors::structure_to_line_descriptors,
        },
        helpers::helpers::{detokenize, extract_data_types_from_token, lex_to_token, print_error},
    },
    types::types::{LineDescriptions, Mapping, MappingValue, Token},
};

impl LineDescriptions {
    fn to_string(self) -> String {
        format!("{}&=>{}%%", self.text, self.line)
    }

    fn to_struct(value: String) -> Vec<Self> {
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

    fn from_token_to_string(token: &Token) -> String {
        return detokenize(&token);
    }

    fn to_token(input: &str) -> Vec<Token> {
        let mut lex: Vec<String> = Vec::new();
        let mut combined_char = String::new();
        let mut lexems: Vec<Token> = Vec::new();
        let mut opened_quotations = 0;
        let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
        for (index, character) in input.chars().enumerate() {
            // println!("{}", input);
            if character == '"' || character == '\'' {
                if opened_quotations == 0 {
                    opened_quotations += 1;
                } else {
                    opened_quotations = 0
                }
            }
            if character.is_whitespace() && !combined_char.trim().is_empty() {
                if opened_quotations > 0 {
                    combined_char.push_str(character.to_string().as_str())
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
                        combined_char.push_str(character.to_string().as_str())
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
                // if let Some(_next) = input.chars().nth(index + 1) {
                //     let mut joined = String::from(combined_char.clone());
                //     joined.push(character);
                //     joined.push(_next);

                //     if [DATA_TYPES.to_vec()]
                //         .concat()
                //         .concat()
                //         .contains(&joined.as_str().trim())
                //     {
                //         combined_char.push_str(character.to_string().as_str())
                //     } else {
                //         panic!("here {}", joined);
                //         lex.push(combined_char.trim().to_string());
                //         combined_char.clear();
                //     }
                // } else {
                if let Some(_) = identifier_regex.find(character.to_string().as_str()) {
                    combined_char.push_str(character.to_string().as_str());
                } else {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
                // }
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
        // println!("{:?}", lexems)
        lexems
    }
}

fn main() {
    let start_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    /* GET ENVIRONMENT ARGUMENTS */
    let args: Vec<String> = env::args().collect();

    /* LINES DESCRIPTION CONTAINING LINE NUMBER */
    let mut lines_: Vec<LineDescriptions> = Vec::new();
    let mut stripped_comments = String::new();
    let mut file_contents = String::new();
    process_file_contents(args, &mut file_contents);

    structure_to_line_descriptors(&file_contents, &mut lines_);
    strip_comments(&lines_, &mut stripped_comments);
    let structured_stripped_compilable_contents: Vec<LineDescriptions> =
        LineDescriptions::to_struct(stripped_comments);
    let extracted_enums = extract_enum(&structured_stripped_compilable_contents);

    let structs_tree = extract_struct(&structured_stripped_compilable_contents);
    let struct_identifiers: Vec<&str> = structs_tree
        .iter()
        .map(|pred| pred.identifier.as_str())
        .collect();

    let enum_identifiers: Vec<&str> = extracted_enums
        .iter()
        .map(|pred| pred.identifier.as_str())
        .collect();

    let custom_data_types_identifiers: Vec<&str> =
        [enum_identifiers.clone(), struct_identifiers].concat();

    let (global_variables, custom_errors) = extract_global_variables(
        &structured_stripped_compilable_contents,
        &custom_data_types_identifiers,
        &enum_identifiers,
    );
    let functions = extract_functions(
        &structured_stripped_compilable_contents,
        &custom_data_types_identifiers,
        &global_variables,
        &enum_identifiers,
    );

    println!(
        "===> STRUCT ===>\n{:#?}\n\n ===> GLOBAL_VARIABLES ===>\n{:#?}\n\n ===> ENUMS ===>\n{:#?}\n\n ===>> CUSTOM_ERRORS ==>>\n{:#?}\n\n ===>> FUNCTIONS ==>>\n{:#?}",
        structs_tree, global_variables, extracted_enums, custom_errors,functions
    );

    let end_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!(
        "Program completed in \x1b[93m{:?}\x1b[0m",
        (end_time.unwrap() - start_time.unwrap())
    );
}

impl Mapping {
    fn new() -> Self {
        Self {
            key: None,
            value: None,
        }
    }

    fn insert(&mut self, key: Option<String>, value: Option<MappingValue>) {
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
