use crate::mods::{
    constants::constants::{DATA_TYPES, SYMBOLS},
    functions::helpers::helpers::{detokenize, print_error, validate_variable},
    types::types::{
        CustomErrorIdentifier, EventIdentifier, EventIdentifierVariants, LineDescriptions,
        MappingIdentifier, OpenedBraceType, Token, VariableIdentifier,
    },
};

pub fn extract_global_elements(
    data: &Vec<LineDescriptions>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    variable_positions: Vec<Option<u8>>,
) -> (
    Vec<VariableIdentifier>,
    Vec<CustomErrorIdentifier>,
    Vec<MappingIdentifier>,
    Vec<EventIdentifier>,
) {
    let mut global_variables: Vec<VariableIdentifier> = Vec::new();
    let mut custom_errors: Vec<CustomErrorIdentifier> = Vec::new();
    let mut events: Vec<EventIdentifier> = Vec::new();
    let mut mappings: Vec<MappingIdentifier> = Vec::new();

    let mut opened_braces = 0;
    let mut opened_brace_type = OpenedBraceType::None;
    let mut variables: Vec<LineDescriptions> = Vec::new();
    let mut combo = String::new();
    for sst in data {
        if sst.text.starts_with("contract") {
            opened_brace_type = OpenedBraceType::Contract;
        } else if sst.text.starts_with("abstract") {
            opened_brace_type = OpenedBraceType::Abstract;
        } else if sst.text.starts_with("constructor") {
            opened_brace_type = OpenedBraceType::Callback;
        } else if sst.text.starts_with("fallback") {
            opened_brace_type = OpenedBraceType::Callback;
        } else if sst.text.starts_with("receive") {
            opened_brace_type = OpenedBraceType::Callback;
        } else if sst.text.starts_with("cron") {
            opened_brace_type = OpenedBraceType::Callback;
        } else if sst.text.starts_with("function") {
            opened_brace_type = OpenedBraceType::Function;
        } else if sst.text.starts_with("modifier") {
            opened_brace_type = OpenedBraceType::Modifier;
        }

        if sst.text.contains("{") {
            for llm in sst.text.chars() {
                if llm == '{' {
                    opened_braces += 1;
                }
            }
        }

        if sst.text.contains("}") {
            for llm in sst.text.chars() {
                if llm == '}' {
                    opened_braces -= 1;
                    if opened_braces == 1 {
                        opened_brace_type = OpenedBraceType::Contract;
                    }
                }
            }
            if sst.text.contains("{") {
                continue;
            }
        }

        if opened_braces == 1 {
            if let OpenedBraceType::Contract | OpenedBraceType::Abstract = opened_brace_type {
                if !sst.text.starts_with("fallback")
                    && !sst.text.starts_with("receive")
                    && !sst.text.starts_with("cron")
                    && !sst.text.starts_with("function")
                    && !sst.text.starts_with("modifier")
                {
                    if !SYMBOLS.contains(&sst.text.as_str()) {
                        if !sst.text.starts_with("contract") {
                            let splited: Vec<&str> = sst.text.split(";").collect();
                            let mut local_combo = String::new();
                            if splited[splited.len() - 1].trim().is_empty() {
                                if !combo.trim().is_empty() {
                                    local_combo.push_str(&combo);
                                }
                                for spl in &splited {
                                    if !spl.trim().is_empty() {
                                        local_combo.push_str(spl);
                                    } else {
                                        local_combo.push(';');
                                    }
                                }

                                variables.push(LineDescriptions {
                                    text: local_combo,
                                    line: sst.line,
                                });
                                combo.clear();
                            } else {
                                combo.push_str(&sst.text);
                            }
                        }
                    }
                }
            }
        }
    }

    if !combo.trim().is_empty() {
        print_error(&format!(
            "Unprocessible entity for {combo}. expecting \";\""
        ));
    }
    let mut skip = 0;
    for variable in variables {
        // println!("{:?}", variable);
        let validated: (
            Option<VariableIdentifier>,
            Option<String>,
            Option<MappingIdentifier>,
            Option<String>,
        );

        if variable_positions.is_empty() {
            validated = validate_variable(variable, custom_data_types, enums, false, None);
        } else {
            for (__index, __position) in variable_positions[skip..].iter().enumerate() {
                if __position.is_some() {
                    skip += __index + 1;
                    break;
                } else {
                }
            }
            validated = validate_variable(
                variable,
                custom_data_types,
                enums,
                false,
                variable_positions[skip - 1],
            );
        };
        if let Some(_raw) = validated.0 {
            global_variables.push(_raw);
        } else if let Some(_custom_err) = validated.1 {
            let structured = process_custom_error(_custom_err);
            custom_errors.push(structured)
        } else if let Some(_mapping) = validated.2 {
            mappings.push(_mapping)
        } else if let Some(_event) = validated.3 {
            let structured = process_event(_event);
            events.push(structured)
        }
    }

    (global_variables, custom_errors, mappings, events)
}

pub fn process_custom_error(_custom_err: String) -> CustomErrorIdentifier {
    let _custom_errors_tokens: Vec<Token> = LineDescriptions::to_token(&_custom_err);
    let mut error_identifier = String::new();
    let mut args: Vec<String> = Vec::new();
    match &_custom_errors_tokens[0] {
        Token::Error => (),
        _other => print_error(&format!(
            "Expecting \"error\" but found {}",
            detokenize(&_other)
        )),
    }

    match &_custom_errors_tokens[1] {
        Token::Identifier(___identifier) => {
            error_identifier.push_str(&___identifier);
        }
        _ => print_error("Unprocessible entity for custom error"),
    }

    let _args_definitions = &_custom_errors_tokens[3.._custom_errors_tokens.len() - 2]
        .split(|pred| pred == &Token::Coma)
        .collect::<Vec<_>>();

    for _arg_collection in _args_definitions {
        if _arg_collection.is_empty() {
            continue;
        }

        let mut stringified_variant = String::new();
        if _arg_collection.len() > 1 {
            print_error("Unprocessible entity for error... Only accept type")
        }
        for _arg in _arg_collection.iter() {
            let val = detokenize(_arg);
            if !DATA_TYPES.contains(&val.as_str()) {
                print_error(&format!("Invalid type \"{}\" for custom error", val))
            }
            stringified_variant.push_str(&val);
        }

        args.push(stringified_variant);
    }

    let structured = CustomErrorIdentifier {
        identifier: error_identifier,
        args: if args.is_empty() { None } else { Some(args) },
    };

    structured
}

pub fn process_event(_event: String) -> EventIdentifier {
    let _custom_event_tokens: Vec<Token> = LineDescriptions::to_token(&_event);
    let mut event_identifier = String::new();
    let mut variants: Vec<EventIdentifierVariants> = Vec::new();
    match &_custom_event_tokens[0] {
        Token::Event => (),
        _other => print_error(&format!(
            "Expecting \"event\" but found {}",
            detokenize(&_other)
        )),
    }

    match &_custom_event_tokens[1] {
        Token::Identifier(___identifier) => {
            event_identifier.push_str(&___identifier);
        }
        _ => print_error("Unprocessible entity for custom error"),
    }

    let _args_definitions = &_custom_event_tokens[3.._custom_event_tokens.len() - 2]
        .split(|pred| pred == &Token::Coma)
        .collect::<Vec<_>>();

    for _arg_collection in _args_definitions {
        if _arg_collection.is_empty() {
            continue;
        }

        let mut indexed = false;
        let mut variant = String::new();
        let mut r#type = String::new();
        {
            let val = detokenize(&_arg_collection[0]);
            if !DATA_TYPES.contains(&val.as_str()) {
                print_error(&format!("Invalid type \"{}\" for event", val))
            } else {
                r#type.push_str(&val);
            }
        }

        if _arg_collection.len() == 2 {
            match &_arg_collection[1] {
                Token::Identifier(__id) => {
                    variant.push_str(&__id);
                }
                _other => print_error(&format!(
                    "Unprocessible entity \"{}\" for event",
                    detokenize(&_other)
                )),
            }
        } else if _arg_collection.len() == 3 {
            match &_arg_collection[1] {
                Token::Indexed => {
                    indexed = true;
                }

                _other => print_error(&format!(
                    "Unprocessible entity \"{}\" for event",
                    detokenize(&_other)
                )),
            }

            match &_arg_collection[2] {
                Token::Identifier(__id) => {
                    variant.push_str(&__id);
                }
                _other => print_error(&format!(
                    "Unprocessible entity \"{}\" for event",
                    detokenize(&_other)
                )),
            }
        }
        let structured_variants = EventIdentifierVariants { indexed, variant };
        variants.push(structured_variants);
    }

    let structured = EventIdentifier {
        identifier: event_identifier,
        variants,
    };
    structured
}
