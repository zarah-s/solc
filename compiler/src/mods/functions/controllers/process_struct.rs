use regex::Regex;

use crate::mods::{
    functions::helpers::helpers::{
        detokenize, extract_custom_data_types_tokens, extract_mapping_identifier, print_error,
        validate_identifier_regex,
    },
    types::types::{LineDescriptions, StructIdentifier, StructType, Token, VariantType},
};

pub fn extract_struct(data: &Vec<LineDescriptions>) -> Vec<StructIdentifier> {
    let extracted_structs = extract_custom_data_types_tokens(&Token::Struct, data);
    let mut struct_identifier: Vec<StructIdentifier> = Vec::new();
    for struct_inst in extracted_structs {
        let mut _identifier: Option<String> = None;
        let mut is_storage: bool = false;
        if let Token::Identifier(_id) = &struct_inst[1] {
            _identifier = Some(_id.to_string());
        } else {
            print_error("Missing struct identifier!!");
        }

        let stripped = &struct_inst[3..struct_inst.len() - 1];
        let splited: Vec<&[Token]> = stripped.split(|pred| pred == &Token::SemiColon).collect();
        let mut combined_types: Vec<StructType> = Vec::new();
        let mut skip = 0;
        for (index, splited_param) in splited.iter().filter(|pred| !pred.is_empty()).enumerate() {
            if skip > index {
                continue;
            }
            let mut type_: Option<String> = None;
            let mut name_: Option<String> = None;
            let mut is_array = false;
            let mut size: Option<String> = None;
            if !splited_param.is_empty() {
                if splited_param.len() < 2 {
                    print_error(&format!("Invalid Struct variants",))
                }

                type_ = Some(format!(
                    "{}",
                    LineDescriptions::from_token_to_string(&splited_param[0],)
                ));

                if let Token::OpenSquareBracket = &splited_param[1] {
                    is_array = true;
                    let close_index = splited_param
                        .iter()
                        .position(|pred| pred == &Token::CloseSquareBracket);

                    if let None = close_index {
                        print_error(&format!(
                            "Syntax error... Expecting a close bracket for struct",
                        ))
                    } else {
                        if close_index.unwrap() - 1 != 1 {
                            match &splited_param[2] {
                                Token::Identifier(_val) => {
                                    if let Some(_dd) = Regex::new(r"^\d+$").unwrap().find(_val) {
                                        if _val == "0" {
                                            print_error(&format!("Invalid array size {}", _val))
                                        }
                                        size = Some(_val.to_owned());
                                    } else {
                                        print_error(&format!("Invalid array size {}", _val))
                                    }
                                }
                                _ => {
                                    let mut _opened_count = 1;
                                    let mut combo = String::new();
                                    for __spl in &splited_param[2..] {
                                        skip = index + 1 + skip;
                                        if let Token::OpenSquareBracket = __spl {
                                            _opened_count += 1;
                                        } else if let Token::CloseSquareBracket = __spl {
                                            _opened_count -= 1;
                                            if _opened_count == 0 {
                                                break;
                                            }
                                        } else {
                                            combo.push_str(&detokenize(__spl));
                                        }
                                    }
                                    size = Some(combo);
                                }
                            }

                            match &splited_param[skip + 2] {
                                Token::Identifier(_val) => name_ = Some(_val.to_owned()),

                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting identifier but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[4])
                                )),
                            }
                        } else {
                            if splited_param.len() != 4 {
                                print_error(&format!("Syntax error on struct"));
                            }
                            match &splited_param[3] {
                                Token::Identifier(_val) => name_ = Some(_val.to_owned()),

                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting identifier but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[3])
                                )),
                            }
                        }
                    }
                } else if let Token::Identifier(_identifier) = &splited_param[1] {
                    if validate_identifier_regex(&_identifier, 0) {
                        name_ = Some(_identifier.to_owned());
                    }
                } else if let Token::Mapping = splited_param[0] {
                    let structured_mapping: crate::mods::types::types::MappingIdentifier =
                        extract_mapping_identifier(
                            &vec![splited_param.to_vec(), vec![Token::SemiColon]].concat(),
                        );
                    combined_types.push(StructType::Mapping(structured_mapping));
                    is_storage = true;
                } else {
                    print_error(&format!("Invalid struct",))
                }
            }

            if name_.is_some() {
                let structured = StructType::Variant(VariantType {
                    is_array,
                    name_: name_.unwrap(),
                    size,
                    type_: type_.unwrap(),
                });
                combined_types.push(structured);
            }
        }

        let struct_build = StructIdentifier {
            identifier: _identifier.unwrap(),
            types: combined_types,
            is_storage,
        };
        struct_identifier.push(struct_build);
    }

    struct_identifier
}
