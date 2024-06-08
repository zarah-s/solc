use std::process;

use crate::mods::{
    constants::constants::{DATA_TYPES, KEYWORDS, SYMBOLS},
    functions::{
        controllers::{
            process_enum::extract_enum,
            process_state_variables::{
                extract_global_elements, process_custom_error, process_event,
            },
        },
        helpers::helpers::{
            detokenize, extract_data_location_from_token, extract_integer_types_from_token,
            lex_to_token, print_error, validate_expression, validate_identifier_regex,
            validate_variable,
        },
    },
    types::types::{
        Argument, Assert, CallIdentifier, CallIdentifierType, ConditionalType, Conditionals,
        ConstructorIdentifier, ConstructorInheritanceInitialization, ContractHeader, ContractType,
        CronIdentifier, CustomErrorIdentifier, Delete, ElIf, EnumIdentifier, EventEmitter,
        EventIdentifier, FallbackIdentifier, FunctionArm, FunctionArmType, FunctionCall,
        FunctionCallType, FunctionHeader, FunctionIdentifier, FunctionMutability,
        FunctionsIdentifier, InterfaceIdentifier, InterfaceVariants, LibraryIdentifier,
        LineDescriptions, Loop, LoopType, MappingAssign, MappingIdentifier, ModifierCall,
        ModifierIdentifier, OpenedBraceType, ReceiveIdentifier, Require, Return, ReturnType,
        Revert, RevertType, StructIdentifier, TerminationType, Token, TuppleAssignment,
        VariableAssign, VariableAssignOperation, VariableAssignType, VariableIdentifier,
        VariableType,
    },
};

use super::process_struct::extract_struct;

pub fn extract_functions(
    data: &Vec<LineDescriptions>,
    custom_data_types: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    enums: &Vec<&str>,
    mappings: &Vec<MappingIdentifier>,
    interfaces: &mut Vec<InterfaceIdentifier>,
    compiled_interfaces: &mut Vec<String>,
    libraries_: &Vec<LibraryIdentifier>,
) -> (
    Vec<FunctionsIdentifier>,
    ContractHeader,
    Vec<LibraryIdentifier>,
) {
    let mut opened_braces = 0;
    let mut libraries: Vec<LibraryIdentifier> = Vec::new();
    let mut opened_braces_type = OpenedBraceType::None;
    let mut processed_data: Vec<Vec<LineDescriptions>> = Vec::new();
    let mut combined = Vec::new();
    let mut function_identifiers: Vec<FunctionsIdentifier> = Vec::new();
    let mut contract_definition = String::new();
    let mut contract_name = String::new();
    let mut contract_type = ContractType::None;
    let mut contract_inheritance: Vec<String> = Vec::new();
    for line in data {
        let raw = &line.text;

        if raw.starts_with("function") {
            if let OpenedBraceType::Library = opened_braces_type {
                opened_braces_type = OpenedBraceType::Library;
            } else if let OpenedBraceType::Contract | OpenedBraceType::Abstract = opened_braces_type
            {
                opened_braces_type = OpenedBraceType::Function;
            }
        } else if raw.starts_with("constructor") {
            opened_braces_type = OpenedBraceType::Constructor;
        } else if raw.starts_with("receive") {
            opened_braces_type = OpenedBraceType::Receive;
        } else if raw.starts_with("fallback") {
            opened_braces_type = OpenedBraceType::Fallback;
        } else if raw.starts_with("cron") {
            opened_braces_type = OpenedBraceType::Cron;
        } else if raw.starts_with("contract") {
            opened_braces_type = OpenedBraceType::Contract;
        } else if raw.starts_with("abstract") {
            opened_braces_type = OpenedBraceType::Abstract;
        } else if raw.starts_with("modifier") {
            if let OpenedBraceType::Library = opened_braces_type {
                opened_braces_type = OpenedBraceType::Library;
            } else {
                opened_braces_type = OpenedBraceType::Modifier;
            }
        } else if raw.starts_with("interface") {
            opened_braces_type = OpenedBraceType::Interface;
        } else if raw.starts_with("library") {
            opened_braces_type = OpenedBraceType::Library;
        }

        if let OpenedBraceType::Contract | OpenedBraceType::Abstract = opened_braces_type {
            contract_definition.push_str(&raw);
        }

        if raw.contains("{") {
            let characters = raw.chars();
            for character in characters {
                if character == '{' {
                    if let OpenedBraceType::Contract | OpenedBraceType::Abstract =
                        opened_braces_type
                    {
                        let lexems = LineDescriptions::to_token(raw);
                        if opened_braces == 0 {
                            extract_contract_headers(
                                lexems,
                                &mut contract_name,
                                &mut contract_inheritance,
                                &mut contract_type,
                            );
                        }
                    }

                    opened_braces += 1;
                }
            }
        }

        if raw.contains("}") {
            for character in raw.chars() {
                if character == '}' {
                    opened_braces -= 1;
                    if opened_braces == 1 {
                        if let OpenedBraceType::Library = opened_braces_type {
                            // TODO: NOTHING
                        } else {
                            if let OpenedBraceType::Function
                            | OpenedBraceType::Constructor
                            | OpenedBraceType::Fallback
                            | OpenedBraceType::Cron
                            | OpenedBraceType::Modifier
                            | OpenedBraceType::Receive = opened_braces_type
                            {
                                opened_braces_type = OpenedBraceType::Contract;
                                combined.push(line.clone());

                                processed_data.push(combined.clone());
                                combined.clear();
                            }
                        }
                        break;
                    } else if opened_braces == 0 {
                        if let OpenedBraceType::Interface | OpenedBraceType::Library =
                            opened_braces_type
                        {
                            opened_braces_type = OpenedBraceType::None;
                            combined.push(line.clone());

                            processed_data.push(combined.clone());
                            combined.clear();
                        }

                        break;
                    }
                }
            }
        }

        if let OpenedBraceType::Function
        | OpenedBraceType::Constructor
        | OpenedBraceType::Receive
        | OpenedBraceType::Cron
        | OpenedBraceType::Library
        | OpenedBraceType::Modifier
        | OpenedBraceType::Interface
        | OpenedBraceType::Fallback = opened_braces_type
        {
            combined.push(line.clone())
        }
    }

    let mut stringified = Vec::new();

    for processed in processed_data {
        let mut combined = String::new();
        for prr in processed {
            if KEYWORDS.contains(&prr.text.as_str()) {
                combined.push_str(&format!("{} ", &prr.text));
            } else {
                combined.push_str(&prr.text);
            }
        }

        stringified.push(combined.clone());
        combined.clear();
    }

    for single_stringified in &stringified {
        let tokens = LineDescriptions::to_token(single_stringified.as_str());
        match tokens[0] {
            Token::Function => extract_full_function(
                custom_data_types,
                global_variables,
                enums,
                mappings,
                &tokens,
                &mut function_identifiers,
                libraries_,
            ),

            Token::Library => {
                let mut identifier = String::new();
                let mut library_events: Vec<EventIdentifier> = Vec::new();
                let mut library_custom_error: Vec<CustomErrorIdentifier> = Vec::new();
                let mut library_functions: Vec<FunctionsIdentifier> = Vec::new();
                let mut library_structs: Vec<StructIdentifier> = Vec::new();
                let mut library_variables: Vec<VariableIdentifier> = Vec::new();
                let mut library_enums: Vec<EnumIdentifier> = Vec::new();
                if let Token::Identifier(__identifier) = &tokens[1] {
                    identifier.push_str(&__identifier);
                } else {
                    print_error("Expecting name for library")
                }

                let library_arm = &tokens[3..tokens.len() - 1];
                let mut opened_braces = 0;
                let mut termination_type = TerminationType::None;
                let mut combo: Vec<Vec<Token>> = Vec::new();
                {
                    let mut sub: Vec<Token> = Vec::new();
                    for (index, arg) in library_arm.iter().enumerate() {
                        if let Token::Event | Token::Error = arg {
                            if index > 0 {
                                let prev = library_arm.get(index - 1);
                                if let Some(_prev) = prev {
                                    if let Token::SemiColon | Token::CloseBraces = _prev {
                                        if !sub.is_empty() {
                                            combo.push(sub.clone());
                                            sub.clear();
                                        }
                                    }
                                }
                            }
                            termination_type = TerminationType::Semicolon;
                        } else if let Token::Struct
                        | Token::Modifier
                        | Token::Enum
                        | Token::Function = arg
                        {
                            if index > 0 {
                                let prev = library_arm.get(index - 1);
                                if let Some(_prev) = prev {
                                    if let Token::SemiColon | Token::CloseBraces = _prev {
                                        if !sub.is_empty() {
                                            combo.push(sub.clone());
                                            sub.clear();
                                        }
                                    }
                                }
                            }
                            termination_type = TerminationType::Braces;
                        }

                        sub.push(arg.clone());
                        if let Token::OpenBraces = arg {
                            opened_braces += 1;
                        }

                        if let Token::OpenParenthesis = arg {
                            if let TerminationType::Semicolon = termination_type {
                                opened_braces += 1;
                            }
                        }
                        if let Token::CloseBraces = arg {
                            opened_braces -= 1;
                            if opened_braces == 0 {
                                if let TerminationType::Braces = termination_type {
                                    combo.push(sub.clone());
                                    sub.clear();
                                    termination_type = TerminationType::None;
                                }
                            }
                        }

                        if let Token::SemiColon = arg {
                            if opened_braces - 1 == 0 {
                                if let TerminationType::Semicolon = termination_type {
                                    opened_braces -= 1;
                                    combo.push(sub.clone());
                                    sub.clear();
                                    termination_type = TerminationType::None;
                                }
                            }
                        }
                    }
                }

                let mut skipped_sort: Vec<usize> = Vec::new();

                let mut sorted_: Vec<Vec<Token>> = Vec::new();

                {
                    for (index, __combo) in combo.iter().enumerate() {
                        if let Token::Modifier | Token::Function = &__combo[0] {
                            skipped_sort.push(index);
                        } else {
                            sorted_.push(__combo.clone())
                        }
                    }

                    for _skipped in skipped_sort {
                        sorted_.push(combo[_skipped].clone())
                    }
                }

                drop(combo);

                for arm in sorted_ {
                    match &arm[0] {
                        Token::Error => {
                            let mut stringified = String::new();
                            for __arm in arm {
                                stringified.push_str(&detokenize(&__arm));
                            }
                            let custom_error = process_custom_error(stringified);
                            library_custom_error.push(custom_error);
                        }

                        Token::Modifier => {
                            let library_custom_data_types = [
                                library_enums
                                    .iter()
                                    .map(|pred| pred.identifier.as_str())
                                    .collect::<Vec<_>>(),
                                library_structs
                                    .iter()
                                    .map(|pred| pred.identifier.as_str())
                                    .collect::<Vec<_>>(),
                            ]
                            .concat();
                            process_modifier(
                                &arm,
                                &library_custom_data_types,
                                &library_enums
                                    .iter()
                                    .map(|pred| pred.identifier.as_str())
                                    .collect::<Vec<_>>(),
                                &library_variables,
                                &Vec::new(),
                                &mut library_functions,
                                libraries_,
                            );
                        }
                        Token::Event => {
                            let mut stringified = String::new();
                            for __arm in arm {
                                stringified.push_str(&format!("{} ", detokenize(&__arm)));
                            }
                            let event = process_event(stringified);
                            library_events.push(event);
                        }
                        Token::Struct => {
                            let mut stringified = String::new();
                            for __arm in arm {
                                stringified.push_str(&format!("{} ", detokenize(&__arm)));
                            }

                            let structs = extract_struct(&vec![LineDescriptions {
                                line: 0,
                                text: format!("contract{}{}{}", "{", stringified, "}"),
                            }]);

                            for r#struct in structs {
                                library_structs.push(r#struct);
                            }
                        }

                        Token::Enum => {
                            let mut stringified = String::new();
                            for __arm in arm {
                                stringified.push_str(&format!("{} ", detokenize(&__arm)));
                            }

                            let enums = extract_enum(&vec![LineDescriptions {
                                line: 0,
                                text: format!("contract{}{}{}", "{", stringified, "}"),
                            }]);

                            for r#enum in enums {
                                library_enums.push(r#enum);
                            }
                        }
                        Token::Function => {
                            let library_custom_data_types = [
                                library_enums
                                    .iter()
                                    .map(|pred| pred.identifier.as_str())
                                    .collect::<Vec<_>>(),
                                library_structs
                                    .iter()
                                    .map(|pred| pred.identifier.as_str())
                                    .collect::<Vec<_>>(),
                            ]
                            .concat();
                            extract_full_function(
                                &library_custom_data_types,
                                &library_variables,
                                &library_enums
                                    .iter()
                                    .map(|pred| pred.identifier.as_str())
                                    .collect::<Vec<_>>(),
                                &Vec::new(),
                                &arm,
                                &mut library_functions,
                                libraries_,
                            );
                        }
                        _other => {
                            let mut stringified = String::new();
                            for __arm in &arm {
                                stringified.push_str(&format!("{} ", detokenize(&__arm)));
                            }
                            if DATA_TYPES.contains(&detokenize(&_other).as_str()) {
                                let var = validate_variable(
                                    LineDescriptions {
                                        text: stringified,
                                        line: 0,
                                    },
                                    &Vec::new(),
                                    &Vec::new(),
                                    false,
                                    None,
                                    libraries_,
                                );

                                if var.0.is_none() {
                                    print_error("Unprocessible entity for library constant")
                                }
                                let unwrapped_var = var.0.unwrap();

                                if unwrapped_var.is_array {
                                    print_error("Only constants of value type and byte array type are implemented")
                                }
                                match unwrapped_var.mutability {
                                    Token::Constant => (),
                                    _ => print_error("Only constants of value type and byte array type are implemented")
                                }

                                library_variables.push(unwrapped_var);
                            } else {
                                print_error("Only constants of value type and byte array type are implemented")
                            }
                        }
                    }
                }

                let library_identifier = LibraryIdentifier {
                    identifier,
                    constants: library_variables,
                    custom_errors: library_custom_error,
                    enums: library_enums,
                    events: library_events,
                    functions: library_functions,
                    structs: library_structs,
                };

                libraries.push(library_identifier)
            }

            Token::Interface => {
                let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
                let interface_definition: &[Token] = &tokens[..start_index.unwrap() + 1];
                let mut interface_name = String::new();
                let mut interface_inheritance: Vec<String> = Vec::new();
                extract_contract_headers(
                    interface_definition.to_vec(),
                    &mut interface_name,
                    &mut interface_inheritance,
                    &mut ContractType::Abstract,
                );
                if compiled_interfaces.contains(&interface_name) {
                    continue;
                } else {
                    compiled_interfaces.push(interface_name.to_string())
                }
                let function_body_start_index =
                    tokens.iter().position(|pred| pred == &Token::OpenBraces);
                if let None = function_body_start_index {
                    print_error(&format!("Unprocessible entity",));
                }

                let function_body =
                    &tokens[function_body_start_index.unwrap() + 1..tokens.len() - 1];

                let mut semicolon_seperated: Vec<Vec<Token>> = Vec::new();
                let mut brace_seperated: Vec<Vec<Token>> = Vec::new();
                let mut opened_brace = 0;

                let mut current_variant = InterfaceVariants::None;
                let mut combo: Vec<Token> = Vec::new();
                for (index, tkn) in function_body.iter().enumerate() {
                    match tkn {
                        Token::OpenBraces => {
                            opened_brace += 1;
                            combo.push(tkn.clone());
                        }
                        Token::CloseBraces => {
                            opened_brace -= 1;

                            if opened_brace == 0 {
                                if let InterfaceVariants::Struct | InterfaceVariants::Enum =
                                    current_variant
                                {
                                    combo.push(tkn.clone());
                                    brace_seperated.push(combo.clone());
                                    combo.clear();
                                }
                            }
                        }
                        Token::SemiColon => {
                            if let InterfaceVariants::Error
                            | InterfaceVariants::Function
                            | InterfaceVariants::Event = current_variant
                            {
                                combo.push(Token::SemiColon);
                                semicolon_seperated.push(combo.clone());
                                combo.clear();
                            } else {
                                combo.push(Token::SemiColon);
                            }
                        }
                        Token::Error => {
                            current_variant = InterfaceVariants::Error;
                            combo.push(Token::Error)
                        }
                        Token::Event => {
                            current_variant = InterfaceVariants::Event;
                            combo.push(Token::Event)
                        }
                        Token::Struct => {
                            current_variant = InterfaceVariants::Struct;
                            combo.push(tkn.clone())
                        }
                        Token::Enum => {
                            current_variant = InterfaceVariants::Enum;
                            combo.push(tkn.clone())
                        }
                        Token::Function => {
                            current_variant = InterfaceVariants::Function;
                            combo.push(tkn.clone())
                        }
                        Token::Identifier(_id) => {
                            if opened_brace == 1 {
                                if let InterfaceVariants::Struct = current_variant {
                                    if let Token::SemiColon = function_body.get(index + 1).unwrap()
                                    {
                                        combo.push(Token::Space);
                                        combo.push(tkn.clone());
                                    }
                                } else {
                                    combo.push(tkn.clone());
                                }
                            } else {
                                combo.push(tkn.clone());
                            }
                        }
                        _other => combo.push(_other.clone()),
                    }
                }

                let variants = [semicolon_seperated, brace_seperated].concat();
                let mut interface_events_and_errors: Vec<LineDescriptions> = Vec::new();
                interface_events_and_errors.push(LineDescriptions {
                    line: 0,
                    text: String::from("contract{"),
                });
                let mut interface_enums: Vec<LineDescriptions> = Vec::new();
                interface_enums.push(LineDescriptions {
                    line: 0,
                    text: String::from("contract{"),
                });
                let mut interface_structs: Vec<LineDescriptions> = Vec::new();
                interface_structs.push(LineDescriptions {
                    line: 0,
                    text: String::from("contract{"),
                });
                let mut functions_headers: Vec<FunctionHeader> = Vec::new();
                let mut unprocessed_function_headers: Vec<Vec<Token>> = Vec::new();
                for split in variants {
                    if split.is_empty() {
                        continue;
                    }
                    match split[0] {
                        Token::Error | Token::Event | Token::Enum | Token::Struct => {
                            let mut stringified_data = String::new();

                            for spl_token in &split {
                                stringified_data.push_str(&format!("{} ", &detokenize(&spl_token)));
                            }
                            if let Token::Enum = split[0] {
                                interface_enums.push(LineDescriptions {
                                    line: 0,
                                    text: stringified_data,
                                })
                            } else if let Token::Struct = split[0] {
                                interface_structs.push(LineDescriptions {
                                    line: 0,
                                    text: stringified_data,
                                })
                            } else {
                                interface_events_and_errors.push(LineDescriptions {
                                    line: 0,
                                    text: stringified_data,
                                })
                            }
                        }

                        Token::Function => {
                            unprocessed_function_headers.push(split);
                        }
                        _ => {}
                    }
                }
                interface_events_and_errors.push(LineDescriptions {
                    text: String::from("}"),
                    line: 0,
                });
                interface_enums.push(LineDescriptions {
                    text: String::from("}"),
                    line: 0,
                });
                interface_structs.push(LineDescriptions {
                    text: String::from("}"),
                    line: 0,
                });

                let _structs_tree = extract_struct(&interface_structs);

                let _extracted_enums: Vec<crate::mods::types::types::EnumIdentifier> =
                    extract_enum(&interface_enums);

                let struct_identifiers: Vec<&str> = _structs_tree
                    .iter()
                    .map(|pred| pred.identifier.as_str())
                    .collect();

                let enum_identifiers: Vec<&str> = _extracted_enums
                    .iter()
                    .map(|pred| pred.identifier.as_str())
                    .collect();

                let _custom_data_types_identifiers: Vec<&str> =
                    [enum_identifiers.clone(), struct_identifiers].concat();
                let (_, _custom_errors, _, _events) = extract_global_elements(
                    &interface_events_and_errors,
                    &Vec::new(),
                    &Vec::new(),
                    Vec::new(),
                    libraries_,
                );

                for unprocessed in unprocessed_function_headers {
                    let function_header = extract_function_header(
                        &unprocessed[..],
                        &unprocessed[1],
                        &_custom_data_types_identifiers,
                        &enum_identifiers,
                    );

                    functions_headers.push(function_header);
                }

                let structured = InterfaceIdentifier {
                    custom_errors: _custom_errors,
                    enums: _extracted_enums,
                    events: _events,
                    functions: functions_headers,
                    identifier: interface_name,
                    inheritance: if interface_inheritance.is_empty() {
                        None
                    } else {
                        Some(interface_inheritance)
                    },
                    structs: _structs_tree,
                };
                interfaces.push(structured);
            }
            Token::Modifier => process_modifier(
                &tokens,
                custom_data_types,
                enums,
                global_variables,
                mappings,
                &mut function_identifiers,
                libraries_,
            ),
            Token::Constructor => {
                let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);

                let function_definition: &[Token] = &tokens[..start_index.unwrap()];
                let arguments =
                    prepare_and_get_function_args(function_definition, custom_data_types, enums);
                let close_paren_index = function_definition
                    .iter()
                    .position(|pred| pred == &Token::CloseParenthesis);
                let mut inheritance_initialization: Vec<ConstructorInheritanceInitialization> =
                    Vec::new();
                if let Some(_index) = close_paren_index {
                    let _variants = &function_definition[_index + 1..];
                    if !_variants.is_empty() {
                        let mut _open_brace_count = 0;
                        let mut _full_combo: Vec<Vec<Token>> = Vec::new();
                        let mut _combo: Vec<Token> = Vec::new();
                        for _variant in _variants {
                            match _variant {
                                Token::OpenParenthesis => {
                                    _open_brace_count += 1;
                                    _combo.push(_variant.clone());
                                }
                                Token::CloseParenthesis => {
                                    _open_brace_count -= 1;
                                    _combo.push(_variant.clone());
                                    if _open_brace_count == 0 {
                                        _full_combo.push(_combo.clone());
                                        _combo.clear();
                                    }
                                }
                                _token => {
                                    _combo.push(_token.clone());
                                }
                            }
                        }

                        for __full_combo in _full_combo {
                            let mut args: Vec<String> = Vec::new();
                            let tkns = &__full_combo[2..__full_combo.len() - 1].to_vec();
                            let mut skip = 0;
                            for (index, arg) in tkns.iter().enumerate() {
                                if skip > index {
                                    continue;
                                }
                                match arg {
                                    Token::Identifier(_id) => {
                                        args.push(_id.to_string());
                                    }
                                    Token::Coma => (),
                                    Token::OpenBraces => {
                                        print_error("Named arguments not supported");
                                    }
                                    __other => {
                                        let mut comb = String::new();
                                        let coma_index = &tkns[index..]
                                            .iter()
                                            .position(|pred| pred == &Token::Coma);
                                        if let Some(_index) = coma_index {
                                            for cmb in &tkns[index..index + *_index] {
                                                comb.push_str(&detokenize(cmb))
                                            }
                                            skip = index + *_index;
                                        } else {
                                            for cmb in &tkns[index..tkns.len()] {
                                                comb.push_str(&detokenize(cmb))
                                            }
                                            skip = index + tkns.len();
                                        }

                                        if !comb.trim().is_empty() {
                                            args.push(comb);
                                        }
                                    }
                                }
                            }
                            let structured_initialization = ConstructorInheritanceInitialization {
                                args,
                                identifier: detokenize(&__full_combo[0]),
                            };

                            inheritance_initialization.push(structured_initialization);
                        }
                    }
                }

                let function_body_start_index =
                    tokens.iter().position(|pred| pred == &Token::OpenBraces);
                if let None = function_body_start_index {
                    print_error(&format!("Unprocessible entity",));
                }

                let function_body = &tokens[function_body_start_index.unwrap()..];

                let arms: Vec<FunctionArm> = extract_function_arms(
                    &function_body.to_vec(),
                    &arguments,
                    custom_data_types,
                    global_variables,
                    enums,
                    Vec::new(),
                    mappings,
                    libraries_,
                );

                let structured = ConstructorIdentifier {
                    arguments,
                    arms,
                    initialization: inheritance_initialization,
                };
                function_identifiers.push(FunctionsIdentifier::ConstructorIdentifier(structured))
            }
            Token::Receive => {
                let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
                let function_definition: &[Token] = &tokens[..start_index.unwrap()];
                let arguments =
                    prepare_and_get_function_args(function_definition, custom_data_types, enums);
                if !arguments.is_empty() {
                    print_error("Unprocessible entity for receive function. \"function does not support argument\"")
                }

                if !function_definition.contains(&Token::External) {
                    print_error("Expecting \"external\" for receive function")
                }
                if !function_definition.contains(&Token::Payable) {
                    print_error("Expecting \"payable\" for receive function")
                }

                let function_body_start_index =
                    tokens.iter().position(|pred| pred == &Token::OpenBraces);
                if let None = function_body_start_index {
                    print_error(&format!("Unprocessible entity",));
                }

                let function_body = &tokens[function_body_start_index.unwrap()..];

                let arms: Vec<FunctionArm> = extract_function_arms(
                    &function_body.to_vec(),
                    &arguments,
                    custom_data_types,
                    global_variables,
                    enums,
                    Vec::new(),
                    mappings,
                    libraries_,
                );

                let structured = ReceiveIdentifier { arms };

                function_identifiers.push(FunctionsIdentifier::ReceiveIdentifier(structured))
            }

            Token::Fallback => {
                let mut payable = false;
                let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
                let function_definition: &[Token] = &tokens[..start_index.unwrap()];
                let arguments =
                    prepare_and_get_function_args(function_definition, custom_data_types, enums);
                if !arguments.is_empty() {
                    print_error("Unprocessible entity for fallback function. \"function does not support argument\"")
                }

                if !function_definition.contains(&Token::External) {
                    print_error("Expecting \"external\" for fallback function")
                }
                if function_definition.contains(&Token::Payable) {
                    payable = true;
                }

                let function_body_start_index =
                    tokens.iter().position(|pred| pred == &Token::OpenBraces);
                if let None = function_body_start_index {
                    print_error(&format!("Unprocessible entity"));
                }

                let function_body = &tokens[function_body_start_index.unwrap()..];

                let arms: Vec<FunctionArm> = extract_function_arms(
                    &function_body.to_vec(),
                    &arguments,
                    custom_data_types,
                    global_variables,
                    enums,
                    Vec::new(),
                    mappings,
                    libraries_,
                );

                let structured = FallbackIdentifier { payable, arms };

                function_identifiers.push(FunctionsIdentifier::FallbackIdentifier(structured))
            }
            Token::Cron => {
                let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);

                let function_definition: &[Token] = &tokens[..start_index.unwrap()];

                let args = &function_definition[2..function_definition.len() - 1];
                if args.len() != 3 {
                    print_error("Unprocessible parameters for cron-job");
                }

                let mut min: u8 = 0;
                let mut hr: u8 = 0;
                let mut day: u8 = 0;
                let mut month: u8 = 0;
                let mut timezone: u8 = 0;
                for __arg in &args[1..args.len() - 1] {
                    match __arg {
                        Token::Identifier(_num) => {
                            let nums = _num.split(" ").collect::<Vec<_>>();
                            if nums.len() != 5 {
                                print_error("Unprocessible parameters for cron-job");
                            }
                            for (index, __num) in nums.iter().enumerate() {
                                let num_value = __num.parse::<u8>();
                                match num_value {
                                    Ok(__val) => {
                                        if index == 0 {
                                            if __val > 59 {
                                                print_error("min param ranges from 0-59");
                                            } else {
                                                min = __val;
                                            }
                                        } else if index == 1 {
                                            if __val > 23 {
                                                print_error("hr param ranges from 0-23");
                                            } else {
                                                hr = __val;
                                            }
                                        } else if index == 2 {
                                            if __val == 0 || __val > 31 {
                                                print_error("day param ranges from 1-31");
                                            } else {
                                                day = __val;
                                            }
                                        } else if index == 3 {
                                            if __val == 0 || __val > 12 {
                                                print_error("month param ranges from 1-12");
                                            } else {
                                                month = __val;
                                            }
                                        } else if index == 4 {
                                            timezone = __val;
                                        }
                                    }

                                    Err(_err) => print_error(&format!("{}. for cron params", _err)),
                                }
                            }
                        }
                        _other => print_error(&format!(
                            "Unprocessible cron params {}",
                            detokenize(_other)
                        )),
                    }
                }

                let function_body_start_index =
                    tokens.iter().position(|pred| pred == &Token::OpenBraces);
                if let None = function_body_start_index {
                    print_error(&format!("Unprocessible entity",));
                }

                let function_body = &tokens[function_body_start_index.unwrap()..];
                let arms: Vec<FunctionArm> = extract_function_arms(
                    &function_body.to_vec(),
                    &Vec::new(),
                    custom_data_types,
                    global_variables,
                    enums,
                    Vec::new(),
                    mappings,
                    libraries_,
                );

                let structured = CronIdentifier {
                    arms,
                    day,
                    hr,
                    min,
                    month,
                    timezone,
                };
                function_identifiers.push(FunctionsIdentifier::CronIdentifier(structured));
            }
            _ => (),
        }
    }

    (
        function_identifiers,
        ContractHeader {
            identifier: contract_name,
            inheritance: if contract_inheritance.is_empty() {
                None
            } else {
                Some(contract_inheritance)
            },
            r#type: contract_type,
        },
        libraries,
    )
}

fn process_modifier(
    tokens: &Vec<Token>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    mappings: &Vec<MappingIdentifier>,
    function_identifiers: &mut Vec<FunctionsIdentifier>,
    libraries_: &Vec<LibraryIdentifier>,
) {
    let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
    let mut arguments: Vec<Argument> = Vec::new();
    let mut identifier = String::new();
    let function_definition: &[Token] = &tokens[..start_index.unwrap()];

    match &function_definition[1] {
        Token::Identifier(_identifier) => {
            identifier = _identifier.to_owned();
        }
        _ => {
            print_error("Unprocessible entity for modifier name");
        }
    }
    if let Token::OpenParenthesis = &function_definition[2] {
        arguments = prepare_and_get_function_args(function_definition, custom_data_types, enums);
    }

    let function_body_start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
    if let None = function_body_start_index {
        print_error(&format!("Unprocessible entity",));
    }

    let function_body = &tokens[function_body_start_index.unwrap()..];

    let arms: Vec<FunctionArm> = extract_function_arms(
        &function_body.to_vec(),
        &arguments,
        custom_data_types,
        global_variables,
        enums,
        Vec::new(),
        mappings,
        libraries_,
    );

    let structured = ModifierIdentifier {
        arguments,
        arms,
        name: identifier,
    };
    function_identifiers.push(FunctionsIdentifier::ModifierIdentifier(structured));
}

fn prepare_and_get_function_args(
    function_definition: &[Token],
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
) -> Vec<Argument> {
    let start_params = function_definition
        .iter()
        .position(|pred| pred == &Token::OpenParenthesis);

    let mut open_paren_count = 1;
    let mut pad = 0;
    for check in &function_definition[start_params.unwrap() + 1..] {
        if open_paren_count == 0 {
            break;
        }
        if let Token::OpenParenthesis = check {
            open_paren_count += 1;
        }
        if let Token::CloseParenthesis = check {
            open_paren_count -= 1;
        }
        pad += 1;
    }
    let params_block = &function_definition[start_params.unwrap() + 1..start_params.unwrap() + pad];
    let splited_params_block: Vec<&[Token]> =
        params_block.split(|pred| pred == &Token::Coma).collect();

    let function_arguments = extract_function_params(
        splited_params_block,
        function_definition,
        custom_data_types,
        enums,
    );
    function_arguments
}

fn extract_contract_headers(
    lexems: Vec<Token>,
    contract_name: &mut String,
    contract_inheritance: &mut Vec<String>,
    contract_type: &mut ContractType,
) {
    match &lexems[0] {
        Token::Contract | Token::Interface => match &lexems[1] {
            Token::Identifier(_identifier) => {
                *contract_type = ContractType::Main;
                *contract_name = _identifier.to_owned();
            }
            _ => {
                print_error("Invalid contract identifier");
            }
        },

        Token::Abstract => match &lexems[2] {
            Token::Identifier(_identifier) => {
                *contract_type = ContractType::Abstract;
                *contract_name = _identifier.to_owned();
            }
            _ => {
                print_error("Invalid contract identifier");
            }
        },

        _ => print_error("Unprocessibble entity for contract header"),
    }

    if lexems.contains(&Token::Is) {
        let index_for_is_keyword = lexems.iter().position(|pred| pred == &Token::Is);
        let lexem_slice = &lexems[index_for_is_keyword.unwrap() + 1..&lexems.len() - 1];
        let splits = lexem_slice
            .split(|pred| pred == &Token::Coma)
            .collect::<Vec<_>>()
            .concat();
        for splited in &splits {
            match splited {
                Token::Identifier(_identifier) => contract_inheritance.push(_identifier.clone()),
                _ => {
                    print_error("Unprocessible entity for contract inheritance");
                }
            }
        }
    } else {
        if let Token::Contract | Token::Interface = lexems[0] {
            if lexems.len() != 3 {
                if let Token::CloseBraces = lexems[3] {
                    //TODO: NOTHING
                } else {
                    print_error("Unprocessible entity for contract definition");
                }
            }
        } else {
            if lexems.len() != 4 {
                if let Token::CloseBraces = lexems[4] {
                    //TODO: NOTHING
                } else {
                    print_error("Unprocessible entity for contract definition");
                }
            }
        }
    }
}

fn extract_full_function(
    custom_data_types: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    enums: &Vec<&str>,
    mappings: &Vec<MappingIdentifier>,
    tokens: &Vec<Token>,
    function_identifiers: &mut Vec<FunctionsIdentifier>,
    libraries_: &Vec<LibraryIdentifier>,
) {
    if let Token::OpenParenthesis = &tokens[2] {
    } else {
        print_error(&format!(
            "Unprocessible function name \"{}\"",
            [
                LineDescriptions::from_token_to_string(&tokens[1]),
                LineDescriptions::from_token_to_string(&tokens[2])
            ]
            .join("")
        ))
    }
    let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);

    let function_definition: &[Token] = &tokens[..start_index.unwrap()];
    let function_header =
        extract_function_header(function_definition, &tokens[1], custom_data_types, enums);
    let function_body_start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
    if let None = function_body_start_index {
        print_error(&format!("Unprocessible entity",));
    }

    let function_body = &tokens[function_body_start_index.unwrap()..];
    let function_args = &function_header.arguments;
    let arms: Vec<FunctionArm> = extract_function_arms(
        &function_body.to_vec(),
        &function_args,
        custom_data_types,
        global_variables,
        enums,
        Vec::new(),
        mappings,
        libraries_,
    );

    let structure: FunctionIdentifier = FunctionIdentifier {
        header: function_header,
        arms,
    };
    function_identifiers.push(FunctionsIdentifier::FunctionIdentifier(structure));
}

fn extract_function_header(
    function_definition: &[Token],
    name: &Token,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
) -> FunctionHeader {
    let function_name = match &name {
        Token::Identifier(_val) => {
            let validated = validate_identifier_regex(_val, 0);
            if validated {
                _val
            } else {
                process::exit(1)
            }
        }
        Token::Call | Token::Delegatecall => {
            let validated = validate_identifier_regex(&detokenize(&name), 0);
            if validated {
                if *name == Token::Call {
                    "call"
                } else {
                    "delegatecall"
                }
                // detokenize(&name).to_owned().as_str()
            } else {
                process::exit(1)
            }
        }
        Token::Push => {
            let validated = validate_identifier_regex("push", 0);
            if validated {
                "push"
            } else {
                process::exit(1)
            }
        }

        Token::Pop => {
            let validated = validate_identifier_regex("pop", 0);
            if validated {
                "pop"
            } else {
                process::exit(1)
            }
        }
        _ => {
            print_error(&format!(
                "Unsupported function name \"{}\"",
                LineDescriptions::from_token_to_string(&name)
            ));
            process::exit(1);
        }
    };

    let mut function_modifiers: Vec<ModifierCall> = Vec::new();
    let _modifiers_definition_index = function_definition
        .iter()
        .position(|pred| pred == &Token::CloseParenthesis);

    let __modifiers_definition_count =
        &function_definition[_modifiers_definition_index.unwrap() + 1..];

    let mut skip = 0;
    for (index, __def) in __modifiers_definition_count.iter().enumerate() {
        if skip > index {
            continue;
        }
        match __def {
            Token::Identifier(_identifier) => {
                let next = __modifiers_definition_count.get(index + 1);
                if let Some(_next) = next {
                    if let Token::OpenParenthesis = _next {
                        let __modifier_id = &__modifiers_definition_count[index + 1..];

                        let mut opened_bracket = 0;
                        let mut _collection: Vec<&Token> = Vec::new();
                        for __id in __modifier_id {
                            if let Token::OpenParenthesis = __id {
                                opened_bracket += 1;
                                if opened_bracket > 1 {
                                    _collection.push(__id);
                                }
                            } else if let Token::CloseParenthesis = __id {
                                opened_bracket -= 1;
                                if opened_bracket == 0 {
                                    break;
                                } else {
                                    _collection.push(__id);
                                }
                            } else {
                                _collection.push(__id);
                            }
                        }

                        let splitted = _collection
                            .split(|pred| pred == &&Token::Coma)
                            .collect::<Vec<_>>();
                        let mut __modifier_args: Vec<String> = Vec::new();
                        for __lex_collection in splitted {
                            if __lex_collection.is_empty() {
                                continue;
                            }

                            let mut _stringified = String::new();
                            for _lex in __lex_collection {
                                _stringified.push_str(&detokenize(_lex));
                            }

                            if _stringified.is_empty() {
                                continue;
                            }

                            __modifier_args.push(_stringified);
                        }
                        function_modifiers.push(ModifierCall {
                            identifier: _identifier.to_owned(),
                            arguments: Some(__modifier_args),
                        });
                        skip = index + _collection.len() + 2;
                    } else {
                        function_modifiers.push(ModifierCall {
                            identifier: _identifier.to_owned(),
                            arguments: None,
                        })
                    }
                } else {
                    function_modifiers.push(ModifierCall {
                        identifier: _identifier.to_owned(),
                        arguments: None,
                    })
                }
            }
            _ => (),
        }
    }

    let mut function_override: bool = false;
    let mut function_virtual: bool = false;
    let mut gasless: bool = false;
    let mut function_mutability = FunctionMutability::Mutable;
    let mut function_visibility = Token::Internal;
    let mut function_returns: Option<Vec<ReturnType>> = None;
    let function_arguments =
        prepare_and_get_function_args(function_definition, custom_data_types, enums);

    for visibility in [
        Token::Internal,
        Token::External,
        Token::Public,
        Token::Private,
    ] {
        if function_definition.contains(&visibility) {
            function_visibility = visibility;
        }
    }

    let returns_start_index = function_definition
        .iter()
        .position(|pred| pred == &Token::Returns);

    if let Some(_returns_start_index) = returns_start_index {
        let returns_definition = &function_definition[_returns_start_index..];
        let end_index = returns_definition
            .iter()
            .position(|pred| pred == &Token::CloseParenthesis);
        if let None = end_index {
            let msg: Vec<String> = returns_definition
                .iter()
                .map(|pred| LineDescriptions::from_token_to_string(pred))
                .collect();
            let stringified_function_identifier: Vec<String> = function_definition
                .iter()
                .map(|pred| LineDescriptions::from_token_to_string(pred))
                .collect();

            print_error(&format!(
                "Unprocessible entity {:?} on {}",
                msg.join(" "),
                stringified_function_identifier.join(" ")
            ))
        }

        let splited_returns_block: Vec<&[Token]> = function_definition
            [_returns_start_index + 2..end_index.unwrap() + _returns_start_index]
            .split(|pred| pred == &Token::Coma)
            .collect();
        function_returns = Some(extract_return_types(
            splited_returns_block,
            function_definition,
            custom_data_types,
            enums,
        ));
    }

    if function_definition.contains(&Token::View) {
        function_mutability = FunctionMutability::View;
    } else if function_definition.contains(&Token::Pure) {
        function_mutability = FunctionMutability::Pure;
    }

    if function_definition.contains(&Token::Override) {
        function_override = true;
    }

    if function_definition.contains(&Token::Virtual) {
        function_virtual = true;
    }

    if function_definition.contains(&Token::Gasless) {
        if let FunctionMutability::Mutable = function_mutability {
            gasless = true;
        } else {
            print_error("cannot define \"gasless\" for view or pure function");
        }
    }

    let structured = FunctionHeader {
        gasless,
        mutability: function_mutability,
        name: function_name.to_string(),
        r#override: function_override,
        returns: function_returns,
        r#virtual: function_virtual,
        visibility: function_visibility,
        arguments: function_arguments,
        modifiers: if function_modifiers.is_empty() {
            None
        } else {
            Some(function_modifiers)
        },
    };

    structured
}

fn extract_function_params(
    splited_params_block: Vec<&[Token]>,
    function_definition: &[Token],
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
) -> Vec<Argument> {
    let mut function_arguments: Vec<Argument> = Vec::new();

    for splited_param in splited_params_block {
        if !splited_param.is_empty() {
            let mut type_: Option<String> = None;
            let mut name_: Option<String> = None;
            let mut location_: Option<Token> = None;
            let mut payable_address: bool = false;
            let mut is_array = false;
            let mut is_primitive = true;
            let mut size: Option<String> = None;
            let vec_: Vec<String> = function_definition
                .iter()
                .map(|pred| LineDescriptions::from_token_to_string(pred))
                .collect();
            if !splited_param.is_empty() {
                if splited_param.len() < 2 {
                    print_error(&format!("Invalid function argument {}", vec_.join(" ")))
                }

                if DATA_TYPES
                    .contains(&LineDescriptions::from_token_to_string(&splited_param[0]).as_str())
                {
                    if let Token::String | Token::Bytes = splited_param[0] {
                        is_primitive = false;
                    }
                    type_ = Some(format!(
                        "{}",
                        LineDescriptions::from_token_to_string(&splited_param[0],)
                    ));
                } else if custom_data_types
                    .contains(&LineDescriptions::from_token_to_string(&splited_param[0]).as_str())
                {
                    if !enums.contains(&detokenize(&splited_param[0]).as_str()) {
                        is_primitive = false;
                    }

                    type_ = Some(format!(
                        "{}",
                        LineDescriptions::from_token_to_string(&splited_param[0],)
                    ));
                } else {
                    print_error(&format!(
                        "Unprocessible entity \"{}\"",
                        &LineDescriptions::from_token_to_string(&splited_param[0])
                    ))
                }

                if let Token::OpenSquareBracket = &splited_param[1] {
                    is_array = true;
                    is_primitive = false;
                    let close_index = splited_param
                        .iter()
                        .position(|pred| pred == &Token::CloseSquareBracket);

                    if let None = close_index {
                        print_error(&format!(
                            "Syntax error... Expecting a close bracket for {}",
                            vec_.join(" ")
                        ))
                    } else {
                        let slice = &splited_param[2..close_index.unwrap()];
                        if !slice.is_empty() {
                            let mut expression = String::new();
                            for slc in slice {
                                let detokenized = LineDescriptions::from_token_to_string(slc);
                                expression.push_str(&detokenized);
                            }

                            size = validate_expression(
                                &expression,
                                LineDescriptions {
                                    line: 0,
                                    text: "".to_string(),
                                },
                            );
                        }
                    }
                } else if let Some(_location) = extract_data_location_from_token(&splited_param[1])
                {
                    location_ = Some(_location);
                } else if let Token::Identifier(_identifier) = &splited_param[1] {
                    if validate_identifier_regex(&_identifier, 0) {
                        name_ = Some(_identifier.to_owned());
                    }
                } else if let Token::Payable = &splited_param[1] {
                    if let Token::Address = &splited_param[0] {
                        payable_address = true;
                    } else {
                        print_error(&format!(
                            "Invalid function argument. Payable for a non address type {}",
                            vec_.join(" ")
                        ))
                    }
                } else {
                    print_error(&format!("Invalid function argument {}", vec_.join(" ")))
                }
            }

            match &splited_param[splited_param.len() - 1] {
                Token::Identifier(_val) => name_ = Some(_val.to_owned()),

                _ => print_error(&format!(
                    "Unprocessible entity.. Expecting identifier but found {}",
                    LineDescriptions::from_token_to_string(&splited_param[5])
                )),
            }

            if let None = name_ {
                print_error(&format!(
                    "Syntax error... missing argument identifier {:?} ",
                    vec_.join(" "),
                ))
            }

            if is_array || !is_primitive {
                if splited_param.contains(&Token::Memory) {
                    location_ = Some(Token::Memory);
                } else if splited_param.contains(&Token::Calldata) {
                    location_ = Some(Token::Calldata);
                } else {
                    print_error(&format!(
                        "Expecting \"memory\" or \"calldata\". {} ",
                        vec_.join(" "),
                    ))
                }
            }

            if is_primitive {
                if location_.is_some() {
                    print_error("Cannot declare \"memory\" or \"calldata\" to a primitive type")
                }
            }

            let structured = Argument {
                location: location_,
                name_: name_.unwrap(),
                type_: type_.unwrap(),
                is_array,
                size,
                payable_address,
            };

            function_arguments.push(structured);
        }
    }

    function_arguments
}

fn extract_return_types(
    splited_params_block: Vec<&[Token]>,
    function_definition: &[Token],
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
) -> Vec<ReturnType> {
    let mut function_arguments: Vec<ReturnType> = Vec::new();

    for splited_param in splited_params_block {
        if !splited_param.is_empty() {
            let mut type_: Option<String> = None;
            let mut location_: Option<Token> = None;
            let mut is_array = false;
            let mut size: Option<String> = None;
            let mut is_primitive = true;

            let vec_: Vec<String> = function_definition
                .iter()
                .map(|pred| LineDescriptions::from_token_to_string(pred))
                .collect();

            if DATA_TYPES
                .contains(&LineDescriptions::from_token_to_string(&splited_param[0]).as_str())
            {
                if let Token::String | Token::Bytes = splited_param[0] {
                    is_primitive = false;
                }
                type_ = Some(format!(
                    "{}",
                    LineDescriptions::from_token_to_string(&splited_param[0],)
                ));
            } else {
                if custom_data_types
                    .contains(&LineDescriptions::from_token_to_string(&splited_param[0]).as_str())
                {
                    if !enums.contains(&detokenize(&splited_param[0]).as_str()) {
                        is_primitive = false;
                    }
                    type_ = Some(format!(
                        "{}",
                        LineDescriptions::from_token_to_string(&splited_param[0],)
                    ));
                } else {
                    print_error(&format!(
                        "Unprocessible entity \"{}\"",
                        &LineDescriptions::from_token_to_string(&splited_param[0])
                    ))
                }
            }

            if splited_param.len() > 1 {
                if let Token::OpenSquareBracket = &splited_param[1] {
                    is_array = true;
                    is_primitive = false;

                    let close_index = splited_param
                        .iter()
                        .position(|pred| pred == &Token::CloseSquareBracket);

                    let slice = &splited_param[2..close_index.unwrap()];
                    if !slice.is_empty() {
                        let mut expression = String::new();
                        for slc in slice {
                            let detokenized = LineDescriptions::from_token_to_string(slc);
                            expression.push_str(&detokenized);
                        }

                        size = validate_expression(
                            &expression,
                            LineDescriptions {
                                line: 0,
                                text: "".to_string(),
                            },
                        );
                    }
                } else if let Some(_location) = extract_data_location_from_token(&splited_param[1])
                {
                } else {
                    print_error(&format!("Invalid function argument {}", vec_.join(" ")))
                }
            }

            if !is_primitive {
                if splited_param.contains(&Token::Memory) {
                    location_ = Some(Token::Memory);
                } else if splited_param.contains(&Token::Calldata) {
                    location_ = Some(Token::Calldata);
                } else {
                    print_error(&format!(
                        "Expecting \"memory\" or \"calldata\". {} ",
                        vec_.join(" "),
                    ))
                }
            }

            if !is_primitive {
                if !splited_param.contains(&Token::Memory)
                    && !splited_param.contains(&Token::Calldata)
                {
                    print_error(&format!(
                        "Expecting \"memory\" or \"calldata\". {} ",
                        vec_.join(" "),
                    ))
                }
            }

            if is_primitive {
                if location_.is_some() {
                    print_error("Cannot declare \"memory\" or \"calldata\" to a primitive type")
                }
            }

            let structured = ReturnType {
                location: location_,
                type_: type_.unwrap(),
                is_array,
                size,
            };

            function_arguments.push(structured);
        }
    }

    function_arguments
}

fn extract_function_arms(
    body: &Vec<Token>,
    function_args: &Vec<Argument>,
    custom_data_types: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    enums: &Vec<&str>,
    local_vars: Vec<&VariableIdentifier>,
    mappings: &Vec<MappingIdentifier>,
    libraries_: &Vec<LibraryIdentifier>,
) -> Vec<FunctionArm> {
    let mut arms: Vec<Vec<&Token>> = Vec::new();
    let mut combined: Vec<&Token> = Vec::new();
    let mut opened_packet = 0;
    let mut packet = FunctionArmType::None;
    let mut prev_packet = FunctionArmType::None;
    let mut global_vars_str: Vec<&str> = Vec::new();
    for ddl in global_variables.iter() {
        global_vars_str.push(&ddl.name)
    }

    if body.is_empty() {
        return Vec::new();
    }

    for token in &body[1..body.len() - 1] {
        combined.push(token);

        match token {
            Token::Require => {
                prev_packet = packet;
                packet = FunctionArmType::Require;
            }

            Token::OpenBraces => {
                opened_packet += 1;
            }
            Token::OpenParenthesis => {
                opened_packet += 1;
            }

            Token::CloseBraces => {
                opened_packet -= 1;
                if opened_packet == 0 {
                    arms.push(combined.clone());
                    combined.clear()
                }
            }
            Token::CloseParenthesis => {
                opened_packet -= 1;
            }
            Token::If => {
                packet = FunctionArmType::Conditional;
            }

            Token::SemiColon => {
                packet = prev_packet;
                prev_packet = FunctionArmType::None;
                if opened_packet == 0 {
                    arms.push(combined.clone());
                    combined.clear()
                }
            }
            Token::Identifier(_id) => {
                if let FunctionArmType::None = packet {
                    if custom_data_types.contains(&detokenize(token).as_str())
                        && global_vars_str.contains(&detokenize(token).as_str())
                    {
                        print_error(&format!("Identifier already declared \"{}\"", _id));
                    }

                    if custom_data_types.contains(&_id.as_str()) {
                        prev_packet = packet;

                        packet = FunctionArmType::StructAssign;
                    } else if global_vars_str.contains(&_id.as_str()) {
                        let local = global_variables
                            .iter()
                            .position(|pred| pred.name == _id.to_owned());
                        if let None = local {
                            print_error(&format!("Identifier \"{}\" not found", _id))
                        } else {
                            if let VariableType::Struct = global_variables[local.unwrap()].type_ {
                                prev_packet = packet;

                                packet = FunctionArmType::StructAssign;
                            } else {
                                prev_packet = packet;

                                packet = FunctionArmType::VariableAssign;
                            }
                        }
                    }
                }
            }

            _other => {}
        }
    }

    if !combined.is_empty() {
        print_error("Missing ;");
    }

    let mut joined_conditionals: Vec<Vec<&Token>> = Vec::new();

    for arm in arms {
        if let Token::Else = &arm[0] {
            let last_index = joined_conditionals.len() - 1;
            for sec in arm {
                joined_conditionals[last_index].push(sec);
            }
        } else if let Token::OpenParenthesis = &arm[0] {
            let equals = arm.iter().find(|pred| pred == &&&Token::Equals);
            if let Some(_) = equals {
                joined_conditionals.push(arm.to_owned());
            } else {
                let last_index = joined_conditionals.len() - 1;
                for sec in arm {
                    joined_conditionals[last_index].push(sec);
                }
            }
        } else {
            joined_conditionals.push(arm.to_owned());
        }
    }

    extract_function_block(
        &joined_conditionals,
        function_args,
        custom_data_types,
        enums,
        global_variables,
        local_vars,
        mappings,
        libraries_,
    )
}
fn extract_function_block(
    arms: &Vec<Vec<&Token>>,
    function_args: &Vec<Argument>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    local_vars: Vec<&VariableIdentifier>,
    mappings: &Vec<MappingIdentifier>,
    libraries_: &Vec<LibraryIdentifier>,
) -> Vec<FunctionArm> {
    let mut full_block: Vec<FunctionArm> = Vec::new();
    for block in arms {
        let initial = block[0];
        match initial {
            Token::Identifier(_identifier) => {
                match block[block.len() - 1] {
                    Token::SemiColon => (),
                    _ => print_error("Missing ;"),
                }
                if custom_data_types.contains(&_identifier.as_str()) {
                    let variable =
                        extract_function_variable(&block, custom_data_types, enums, libraries_);
                    if let None = variable {
                        print_error("OOPS!!!");
                    }

                    full_block.push(FunctionArm::VariableIdentifier(variable.unwrap()));
                } else {
                    if let Token::OpenParenthesis = &block[1] {
                        extract_function_call(_identifier, &mut full_block, block);
                    } else {
                        let mut local_variables_identifiers: Vec<&String> = Vec::new();
                        for ___local_var in &local_vars {
                            local_variables_identifiers.push(&___local_var.name);
                        }

                        for code_block in &full_block {
                            match code_block {
                                FunctionArm::VariableIdentifier(_id) => {
                                    local_variables_identifiers.push(&_id.name);
                                }
                                _ => (),
                            }
                        }

                        if local_variables_identifiers.contains(&_identifier) {
                            let mut local_variable_identifiers = Vec::new();
                            for code_block in &full_block {
                                match code_block {
                                    FunctionArm::VariableIdentifier(_variable_identifier) => {
                                        local_variable_identifiers.push(_variable_identifier);
                                    }
                                    _ => (),
                                }
                            }

                            let joined = [local_variable_identifiers, local_vars.clone()].concat();
                            let var = joined
                                .iter()
                                .find(|pred| pred.name == _identifier.to_string());
                            if let Some(_var) = var {
                                if _var.type_ == VariableType::Contract {
                                    extract_function_call(_identifier, &mut full_block, block);
                                } else {
                                    let function_scope_variable = extract_function_scope_variable(
                                        Some(_var),
                                        None,
                                        block,
                                        _identifier,
                                    );
                                    if let Some(_) = function_scope_variable {
                                        full_block.push(function_scope_variable.unwrap())
                                    }
                                }
                            } else {
                                print_error(&format!("Unidentifined variable \"{}\"", _identifier))
                            }
                        } else {
                            let global_variables_identifiers: Vec<&String> =
                                global_variables.iter().map(|pred| &pred.name).collect();
                            let global_mappings: Vec<String> =
                                mappings.iter().map(|pred| pred.name.to_owned()).collect();
                            if function_args
                                .iter()
                                .map(|pred| &pred.name_)
                                .collect::<Vec<_>>()
                                .contains(&_identifier)
                            {
                                let arg = function_args
                                    .iter()
                                    .find(|pred| pred.name_ == *_identifier)
                                    .unwrap();
                                let mut r#type = VariableType::Variable;

                                if !DATA_TYPES.contains(&arg.type_.as_str()) {
                                    if custom_data_types.contains(&arg.type_.as_str()) {
                                        if enums.contains(&_identifier.as_str()) {
                                            r#type = VariableType::Enum
                                        } else {
                                            r#type = VariableType::Struct
                                        }
                                    } else {
                                        r#type = VariableType::Contract
                                    }
                                }
                                let var = VariableIdentifier {
                                    data_type: lex_to_token(&arg.type_),
                                    index: None,
                                    is_array: arg.is_array,
                                    mutability: if arg.location.is_none() {
                                        Token::Mutable
                                    } else {
                                        if let Token::Memory = arg.location.clone().unwrap() {
                                            Token::Mutable
                                        } else {
                                            Token::Immutable
                                        }
                                    },
                                    name: arg.name_.to_string(),
                                    size: arg.size.clone(),
                                    storage_location: arg.location.clone(),
                                    type_: r#type,
                                    value: None,
                                    visibility: Token::Private,
                                };

                                if let VariableType::Contract = var.type_ {
                                    extract_function_call(_identifier, &mut full_block, block);
                                } else {
                                    if let Token::Address = var.data_type {
                                        if block.contains(&&Token::Call)
                                            || block.contains(&&Token::Delegatecall)
                                        {
                                            extract_low_level_call(
                                                block,
                                                &mut full_block,
                                                arg.payable_address,
                                            );
                                        } else {
                                            let function_scope_variable =
                                                extract_function_scope_variable(
                                                    Some(&var),
                                                    None,
                                                    block,
                                                    _identifier,
                                                );
                                            if let Some(_) = function_scope_variable {
                                                full_block.push(function_scope_variable.unwrap());
                                            }
                                        }
                                    } else if let VariableType::Struct = var.type_ {
                                        if block.contains(&&Token::Call)
                                            || block.contains(&&Token::Delegatecall)
                                        {
                                            extract_low_level_call(
                                                block,
                                                &mut full_block,
                                                arg.payable_address,
                                            );
                                        } else {
                                            let function_scope_variable =
                                                extract_function_scope_variable(
                                                    Some(&var),
                                                    None,
                                                    block,
                                                    _identifier,
                                                );
                                            if let Some(_) = function_scope_variable {
                                                full_block.push(function_scope_variable.unwrap());
                                            }
                                        }
                                    } else {
                                        let function_scope_variable =
                                            extract_function_scope_variable(
                                                Some(&var),
                                                None,
                                                block,
                                                _identifier,
                                            );
                                        if let Some(_) = function_scope_variable {
                                            full_block.push(function_scope_variable.unwrap());
                                        }
                                    }
                                }
                            } else if global_variables_identifiers.contains(&_identifier) {
                                let var =
                                    global_variables.iter().find(|pred: &&VariableIdentifier| {
                                        pred.name == _identifier.to_string()
                                    });
                                if let Some(_var) = var {
                                    if let VariableType::Contract = _var.type_ {
                                        extract_function_call(_identifier, &mut full_block, block);
                                    } else {
                                        let function_scope_variable =
                                            extract_function_scope_variable(
                                                Some(_var),
                                                None,
                                                block,
                                                _identifier,
                                            );
                                        if let Some(_) = function_scope_variable {
                                            full_block.push(function_scope_variable.unwrap());
                                        }
                                    }
                                } else {
                                    print_error(&format!(
                                        "Unidentifined variable \"{}\"",
                                        _identifier
                                    ))
                                }
                            } else if global_mappings.contains(_identifier) {
                                let var = mappings
                                    .iter()
                                    .find(|pred| pred.name == _identifier.to_string());
                                if let Some(_var) = var {
                                    let function_scope_variable = extract_function_scope_variable(
                                        None,
                                        Some(_var),
                                        block,
                                        _identifier,
                                    );
                                    if let Some(_) = function_scope_variable {
                                        full_block.push(function_scope_variable.unwrap());
                                    }
                                }
                            } else if _identifier == "_" {
                                if let Token::SemiColon = block[1] {
                                    full_block.push(FunctionArm::FunctionExecution);
                                } else {
                                    print_error(&format!(
                                        "Unidentifined variable \"{}\"",
                                        _identifier
                                    ))
                                }
                            } else if _identifier == "break" || _identifier == "continue" {
                                if let Token::SemiColon = block[1] {
                                    full_block.push(if _identifier == "break" {
                                        FunctionArm::Break
                                    } else {
                                        FunctionArm::Continue
                                    });
                                } else {
                                    print_error("Expecting \";\" for break");
                                }
                            } else {
                                let variable = extract_function_variable(
                                    &block,
                                    custom_data_types,
                                    enums,
                                    libraries_,
                                );
                                if let None = variable {
                                    print_error("OOPS!!!");
                                }

                                full_block.push(FunctionArm::VariableIdentifier(variable.unwrap()));
                                // print_error(&format!("Unidentifined variable \"{}\"", _identifier))
                            }
                        }
                    }
                }
            }
            Token::Emit => {
                match block[block.len() - 1] {
                    Token::SemiColon => (),
                    _ => {
                        print_error("Expecting \";\"");
                    }
                }
                let mut event_identifier = String::new();
                let mut values: Vec<String> = Vec::new();
                if let Token::Identifier(_identifier) = block[1] {
                    event_identifier.push_str(&_identifier);
                } else {
                    print_error("Unprocessible entity for emit");
                }
                if let Token::OpenParenthesis = block[2] {
                    let splitted_value_fields = &block[3..block.len() - 2]
                        .split(|pred| pred == &&Token::Coma)
                        .collect::<Vec<_>>();

                    for value_field_collection in splitted_value_fields {
                        if value_field_collection.is_empty() {
                            continue;
                        }
                        let mut stringified = String::new();
                        for _value in value_field_collection.iter() {
                            stringified.push_str(&detokenize(_value));
                        }

                        values.push(stringified);
                    }
                } else {
                    print_error("Unprocessible entity for emit");
                }

                let structured = FunctionArm::EventEmitter(EventEmitter {
                    identifier: event_identifier,
                    values,
                });
                full_block.push(structured);
            }
            Token::While => {
                let open_brace_index = block.iter().position(|pred| pred == &&Token::OpenBraces);
                let mut _condition = String::new();
                if let Some(_open_brace_index) = open_brace_index {
                    let condition_block = &block[2.._open_brace_index - 1];
                    for cond in condition_block {
                        _condition.push_str(&detokenize(cond));
                    }

                    let mut batched: Vec<Token> = Vec::new();
                    for _batch in &block[_open_brace_index..] {
                        batched.push(_batch.to_owned().clone());
                    }
                    let mut _local_vars: Vec<&VariableIdentifier> = local_vars.clone();
                    for __blk in &full_block {
                        if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                            _local_vars.push(_identifier)
                        }
                    }

                    let __arms = extract_function_arms(
                        &batched,
                        function_args,
                        custom_data_types,
                        global_variables,
                        enums,
                        _local_vars,
                        mappings,
                        libraries_,
                    );

                    let structured = FunctionArm::Loop(Loop {
                        arms: __arms,
                        condition: _condition,
                        identifier: None,
                        iterator: None,
                        value: None,
                        r#type: LoopType::While,
                    });

                    full_block.push(structured);
                }
            }
            Token::If => {
                let mut opened_paren_condition = 0;

                let mut tree = Conditionals::new(Vec::new());
                let mut batched: Vec<Token> = Vec::new();
                let mut conditional_type = ConditionalType::None;
                let mut condition: Vec<Token> = Vec::new();
                let mut skip = 0;
                let mut opened_braces = 0;

                for (index, blk) in block.iter().enumerate() {
                    if index > 0 && skip == index {
                        continue;
                    }
                    if let Token::If = blk {
                        if index > 0 {
                            let backward_index = block.get(index - 1);
                            if let Some(_idx) = backward_index {
                                if let Token::If = _idx {
                                    // TODO: NOTHING
                                } else {
                                    conditional_type = ConditionalType::If;
                                }
                            }
                        } else {
                            conditional_type = ConditionalType::If;
                        }
                    }

                    if let Token::Else = blk {
                        let forward_index = block.get(index + 1);
                        if let Some(_idx) = forward_index {
                            if let Token::If = _idx {
                                conditional_type = ConditionalType::ElIf;
                                skip = index + 1;
                            } else {
                                conditional_type = ConditionalType::El;
                            }
                        }
                    }

                    if let Token::OpenBraces = blk {
                        opened_braces += 1;
                    }

                    if let Token::CloseBraces = blk {
                        opened_braces -= 1;
                    }

                    if opened_braces == 0 {
                        match conditional_type {
                            ConditionalType::If | ConditionalType::ElIf => {
                                if index > 1 {
                                    condition.push(blk.to_owned().clone());
                                }
                                if let Token::OpenParenthesis = blk {
                                    opened_paren_condition += 1;
                                }

                                if let Token::CloseParenthesis = blk {
                                    opened_paren_condition -= 1;
                                }

                                if opened_paren_condition == 0 {
                                    condition.pop();

                                    if !condition.is_empty() {
                                        if let ConditionalType::If = conditional_type {
                                            tree.condition = condition.clone();
                                        } else {
                                            tree.elif.push(ElIf {
                                                arm: Vec::new(),
                                                condition: condition[1..].to_vec().clone(),
                                            });
                                        }
                                        condition.clear();
                                    }
                                }
                            }

                            _ => (),
                        }
                    }

                    if opened_braces > 0 {
                        batched.push(blk.to_owned().clone());
                    }

                    if opened_braces == 0 && index > 0 {
                        match conditional_type {
                            ConditionalType::If => {
                                if !batched.is_empty() {
                                    batched.push(Token::CloseBraces);
                                }
                                let mut _local_vars: Vec<&VariableIdentifier> = local_vars.clone();
                                for __blk in &full_block {
                                    if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                                        _local_vars.push(_identifier)
                                    }
                                }
                                let __arm: Vec<FunctionArm> = extract_function_arms(
                                    &batched,
                                    function_args,
                                    custom_data_types,
                                    global_variables,
                                    enums,
                                    _local_vars,
                                    mappings,
                                    libraries_,
                                );
                                tree.arm = __arm;

                                batched.clear();
                            }

                            ConditionalType::ElIf => {
                                if !batched.is_empty() {
                                    batched.push(Token::CloseBraces);
                                }
                                let mut _local_vars: Vec<&VariableIdentifier> = local_vars.clone();
                                for __blk in &full_block {
                                    if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                                        _local_vars.push(_identifier)
                                    }
                                }
                                let __arm: Vec<FunctionArm> = extract_function_arms(
                                    &batched,
                                    function_args,
                                    custom_data_types,
                                    global_variables,
                                    enums,
                                    _local_vars,
                                    mappings,
                                    libraries_,
                                );

                                let last_len = tree.elif.len();
                                if last_len > 0 {
                                    if !__arm.is_empty() {
                                        tree.elif[last_len - 1].arm = __arm;
                                    }
                                }
                                batched.clear();
                            }

                            ConditionalType::El => {
                                if !batched.is_empty() {
                                    batched.push(Token::CloseBraces);
                                }

                                let mut _local_vars: Vec<&VariableIdentifier> = local_vars.clone();
                                for __blk in &full_block {
                                    if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                                        _local_vars.push(_identifier)
                                    }
                                }

                                let __arm: Vec<FunctionArm> = extract_function_arms(
                                    &batched,
                                    function_args,
                                    custom_data_types,
                                    global_variables,
                                    enums,
                                    _local_vars,
                                    mappings,
                                    libraries_,
                                );
                                tree.el = Some(__arm);
                                batched.clear();
                            }
                            _ => (),
                        }
                    }
                }

                let structure = FunctionArm::Conditionals(tree);

                full_block.push(structure);
            }

            Token::Delete => match block[1] {
                Token::Identifier(_identifier) => {
                    match block[block.len() - 1] {
                        Token::SemiColon => (),
                        _ => print_error("Missing ;"),
                    }

                    let mut local_variables_identifiers: Vec<&String> = Vec::new();

                    for code_block in &full_block {
                        match code_block {
                            FunctionArm::VariableIdentifier(_id) => {
                                local_variables_identifiers.push(&_id.name);
                            }
                            _ => (),
                        }
                    }

                    if local_variables_identifiers.contains(&_identifier) {
                        let mut local_variable_identifiers = Vec::new();
                        for code_block in &full_block {
                            match code_block {
                                FunctionArm::VariableIdentifier(_variable_identifier) => {
                                    local_variable_identifiers.push(_variable_identifier);
                                }
                                _ => (),
                            }
                        }
                        let var = local_variable_identifiers
                            .iter()
                            .find(|pred| pred.name == _identifier.to_string());
                        if let Some(_var) = var {
                            let close_brack_index = block
                                .iter()
                                .position(|pred| pred == &&Token::CloseSquareBracket);
                            let open_brack_index = block
                                .iter()
                                .position(|pred| pred == &&Token::OpenSquareBracket);
                            let mut array_index: Option<String> = None;
                            if let Some(_open_index) = open_brack_index {
                                if let Some(_close_brack_index) = close_brack_index {
                                    let mut stringified = String::new();
                                    for _str in &block[_open_index + 1.._close_brack_index] {
                                        stringified.push_str(&detokenize(_str));
                                    }
                                    array_index = Some(stringified)
                                } else {
                                    print_error("Missing ]");
                                }
                            }
                            if _var.is_array {
                                full_block.push(FunctionArm::Delete(Delete {
                                    identifier: _identifier.to_string(),
                                    type_: VariableAssignType::Array(array_index),
                                    variants: None,
                                    data_type: _var.data_type.clone(),
                                }))
                            } else if let VariableType::Struct = _var.type_ {
                                if let Token::Dot = block[2] {
                                    let mut variants: Vec<String> = Vec::new();

                                    for _variant in &block[3..block.len() - 1] {
                                        match _variant {
                                            Token::Identifier(__id) => {
                                                variants.push(__id.to_owned());
                                            }
                                            _ => (),
                                        }
                                    }
                                    full_block.push(FunctionArm::Delete(Delete {
                                        identifier: _identifier.to_string(),
                                        type_: VariableAssignType::Struct,
                                        variants: Some(variants),
                                        data_type: _var.data_type.clone(),
                                    }));
                                } else {
                                    full_block.push(FunctionArm::Delete(Delete {
                                        identifier: _identifier.to_string(),
                                        type_: VariableAssignType::Struct,
                                        variants: None,
                                        data_type: _var.data_type.clone(),
                                    }));
                                }
                            } else {
                                full_block.push(FunctionArm::Delete(Delete {
                                    identifier: _identifier.to_string(),
                                    type_: VariableAssignType::Expression,
                                    variants: None,
                                    data_type: _var.data_type.clone(),
                                }))
                            }
                        } else {
                            print_error(&format!("Unidentifined variable \"{}\"", _identifier))
                        }
                    } else {
                        let global_variables_identifiers: Vec<&String> =
                            global_variables.iter().map(|pred| &pred.name).collect();
                        let global_mappings: Vec<String> =
                            mappings.iter().map(|pred| pred.name.to_owned()).collect();
                        if global_variables_identifiers.contains(&_identifier) {
                            let var = global_variables
                                .iter()
                                .find(|pred| pred.name == _identifier.to_string());

                            if let Some(_var) = var {
                                let close_brack_index = block
                                    .iter()
                                    .position(|pred| pred == &&Token::CloseSquareBracket);

                                let open_brack_index = block
                                    .iter()
                                    .position(|pred| pred == &&Token::OpenSquareBracket);
                                let mut array_index: Option<String> = None;
                                if let Some(_open_index) = open_brack_index {
                                    if let Some(_close_brack_index) = close_brack_index {
                                        let mut stringified = String::new();
                                        for _str in &block[_open_index + 1.._close_brack_index] {
                                            stringified.push_str(&detokenize(_str));
                                        }
                                        array_index = Some(stringified)
                                    } else {
                                        print_error("Missing ]");
                                    }
                                }
                                if _var.is_array {
                                    full_block.push(FunctionArm::Delete(Delete {
                                        identifier: _identifier.to_string(),
                                        type_: VariableAssignType::Array(array_index),
                                        variants: None,
                                        data_type: _var.data_type.clone(),
                                    }))
                                } else if let VariableType::Struct = _var.type_ {
                                    if let Token::Dot = block[2] {
                                        let mut variants: Vec<String> = Vec::new();

                                        for _variant in &block[3..block.len() - 1] {
                                            match _variant {
                                                Token::Identifier(__id) => {
                                                    variants.push(__id.to_owned());
                                                }
                                                _ => (),
                                            }
                                        }

                                        full_block.push(FunctionArm::Delete(Delete {
                                            identifier: _identifier.to_string(),
                                            type_: VariableAssignType::Struct,
                                            variants: Some(variants),
                                            data_type: _var.data_type.clone(),
                                        }));
                                    } else {
                                        full_block.push(FunctionArm::Delete(Delete {
                                            identifier: _identifier.to_string(),
                                            type_: VariableAssignType::Struct,
                                            variants: None,
                                            data_type: _var.data_type.clone(),
                                        }));
                                    }
                                } else {
                                    full_block.push(FunctionArm::Delete(Delete {
                                        identifier: _identifier.to_string(),
                                        type_: VariableAssignType::Expression,
                                        variants: None,
                                        data_type: _var.data_type.clone(),
                                    }))
                                }
                            } else {
                                print_error(&format!("Unidentifined variable \"{}\"", _identifier))
                            }
                        } else if global_mappings.contains(_identifier) {
                            let mut variants: Vec<String> = Vec::new();
                            for (index, __variant) in block[2..block.len() - 1].iter().enumerate() {
                                if let Token::OpenSquareBracket = __variant {
                                } else if let Token::CloseSquareBracket = __variant {
                                } else {
                                    match __variant {
                                        Token::Identifier(_id) => {
                                            if index > 2 {
                                                let backward_token =
                                                    block[2..block.len() - 1].get(index - 2);
                                                if let Some(__s) = backward_token {
                                                    match __s {
                                                        Token::Msg => {
                                                            variants.push(format!(
                                                                "msg.{}",
                                                                _id.to_owned()
                                                            ));
                                                        }
                                                        _ => {
                                                            variants.push(_id.to_owned());
                                                        }
                                                    }
                                                } else {
                                                    variants.push(_id.to_owned());
                                                }
                                            } else {
                                                variants.push(_id.to_owned());
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }

                            if variants.is_empty() {
                                print_error(&format!("Missing key for mapping \"{}\"", _identifier))
                            }

                            full_block.push(FunctionArm::Delete(Delete {
                                identifier: _identifier.to_string(),
                                type_: VariableAssignType::Mapping,
                                variants: Some(variants),
                                data_type: Token::Identifier("mapping".to_owned()),
                            }));
                        } else {
                            print_error(&format!("Unidentifined variable \"{}\"", _identifier))
                        }
                    }
                }
                _ => print_error("Invalid delete expression."),
            },
            Token::Require => {
                match block[block.len() - 1] {
                    Token::SemiColon => (),
                    _ => print_error("Missing ;"),
                }
                let split: Vec<&[&Token]> = block.split(|pred| pred == &&Token::Coma).collect();
                let mut condition = String::new();
                let mut message: String = String::new();
                if split.len() == 2 {
                    for cond in &split[0][2..] {
                        condition.push_str(&detokenize(cond))
                    }
                } else {
                    for cond in &split[0][2..split[0].len() - 2] {
                        condition.push_str(&detokenize(cond))
                    }
                }

                if let Some(_) = split.get(1) {
                    for msg in &split[1][..split[1].len() - 2] {
                        message.push_str(&detokenize(msg))
                    }
                }

                full_block.push(FunctionArm::Require(Require {
                    condition,
                    message: if message.trim().is_empty() {
                        None
                    } else {
                        Some(message)
                    },
                }))
            }
            Token::For => {
                let open_brace_index = block.iter().position(|pred| pred == &&Token::OpenBraces);
                let mut _value: Option<String> = None;
                let mut __identifier: String = String::new();
                let mut _condition = String::new();
                let mut _iterator = String::new();
                if let Some(_open_brace_index) = open_brace_index {
                    let condition_block = &block[2.._open_brace_index - 1];

                    let splitted = condition_block
                        .split(|pred| pred == &&Token::SemiColon)
                        .collect::<Vec<_>>();

                    if splitted.len() != 3 {
                        print_error("Unprocessible entity for loop");
                    } else {
                        if let Some(_) = extract_integer_types_from_token(&splitted[0][0]) {
                            match splitted[0][1] {
                                Token::Identifier(_id) => __identifier = _id.to_string(),
                                _ => {
                                    print_error("Unprocessible entity for loop");
                                }
                            }

                            if splitted[0].contains(&&Token::Equals) {
                                let mut _stringified_val = String::new();

                                for __token in &splitted[0][3..] {
                                    _stringified_val.push_str(&detokenize(__token));
                                }
                                _value = Some(_stringified_val);
                            } else {
                                match splitted[0][splitted[0].len() - 1] {
                                    Token::Identifier(_id) => _value = Some("0".to_string()),
                                    _ => {
                                        print_error("Unprocessible entity for loop");
                                    }
                                }
                            }
                        } else {
                            print_error("Identifier type can only be uint or int");
                        }
                    }

                    match splitted[1][0] {
                        Token::Identifier(_id) => {
                            if _id != &__identifier {
                                print_error("Mismatched identifier");
                            } else {
                                for __condition in splitted[1] {
                                    _condition.push_str(&detokenize(__condition));
                                }
                            }
                        }
                        _ => {
                            print_error("Unprocessible entity for loop");
                        }
                    }

                    for __op in splitted[2] {
                        _iterator.push_str(&detokenize(__op));
                    }

                    let mut batched: Vec<Token> = Vec::new();
                    for _batch in &block[_open_brace_index..] {
                        batched.push(_batch.to_owned().clone());
                    }
                    let mut _local_vars: Vec<&VariableIdentifier> = local_vars.clone();
                    for __blk in &full_block {
                        if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                            _local_vars.push(_identifier)
                        }
                    }

                    let __arms = extract_function_arms(
                        &batched,
                        function_args,
                        custom_data_types,
                        global_variables,
                        enums,
                        _local_vars,
                        mappings,
                        libraries_,
                    );

                    let structured = FunctionArm::Loop(Loop {
                        arms: __arms,
                        condition: _condition,
                        identifier: Some(__identifier),
                        iterator: Some(_iterator),
                        value: _value,
                        r#type: LoopType::For,
                    });
                    full_block.push(structured);
                } else {
                    print_error("Unprocessible Entity for for loop");
                }
            }
            Token::Assert => {
                let mut msg: String = String::new();
                let mut opened_count = 0;
                for __blk in block {
                    if opened_count > 0 {
                        if let Token::CloseParenthesis = __blk {
                            // TODO: NOTHING
                        } else {
                            msg.push_str(&detokenize(__blk))
                        }
                    }
                    if let Token::OpenParenthesis = __blk {
                        opened_count += 1;
                    }
                    if let Token::CloseParenthesis = __blk {
                        opened_count -= 1;
                    }
                }

                let structured = Assert { assert: msg };
                full_block.push(FunctionArm::Assert(structured));
            }
            Token::Revert => {
                let mut msg: String = String::new();
                let mut _type: Option<RevertType> = None;
                if let Token::Identifier(_) = block[1] {
                    _type = Some(RevertType::Custom);

                    for __blk in &block[1..block.len() - 1] {
                        msg.push_str(&detokenize(__blk))
                    }
                } else {
                    _type = Some(RevertType::Default);
                    let mut opened_count = 0;
                    for __blk in block {
                        if opened_count > 0 {
                            if let Token::CloseParenthesis = __blk {
                                // TODO: NOTHING
                            } else {
                                msg.push_str(&detokenize(__blk))
                            }
                        }
                        if let Token::OpenParenthesis = __blk {
                            opened_count += 1;
                        }
                        if let Token::CloseParenthesis = __blk {
                            opened_count -= 1;
                        }
                    }
                }

                let structured = Revert {
                    msg,
                    r#type: _type.unwrap(),
                };
                full_block.push(FunctionArm::Revert(structured));
            }
            Token::Return => {
                match block[block.len() - 1] {
                    Token::SemiColon => (),
                    _ => print_error("Missing ;"),
                }
                let mut value = String::new();

                for blk in &block[1..block.len() - 1] {
                    value.push_str(&detokenize(blk))
                }
                full_block.push(FunctionArm::Return(Return { value }));
            }
            _token => {
                match block[block.len() - 1] {
                    Token::SemiColon => (),
                    _ => {
                        print_error("Missing ;");
                    }
                }

                if DATA_TYPES.contains(&detokenize(_token).as_str()) {
                    let mut text = String::new();
                    for strr in block {
                        text.push_str(&format!("{} ", &detokenize(strr)))
                    }
                    let mut is_call = false;
                    if let Token::Address | Token::Msg | Token::Payable = block[0] {
                        if block.contains(&&Token::Call) || block.contains(&&Token::Delegatecall) {
                            is_call = true;
                        }
                    }
                    if is_call {
                        let is_payable = if let Token::Payable = block[0] {
                            true
                        } else {
                            false
                        };
                        extract_low_level_call(block, &mut full_block, is_payable)
                    } else {
                        let variable =
                            extract_function_variable(&block, custom_data_types, enums, libraries_);
                        if let None = variable {
                            print_error("OOPS!!!");
                        }
                        full_block.push(FunctionArm::VariableIdentifier(variable.unwrap()));
                    }
                } else if let Token::OpenParenthesis = _token {
                    let end_index = block
                        .iter()
                        .position(|pred| pred == &&Token::CloseParenthesis);
                    if let Some(_index) = end_index {
                        let vars_ = &block[1.._index];
                        let mut _value: Option<String> = None;

                        {
                            let mut __value = String::new();
                            for __val in &block[_index + 2..block.len() - 1] {
                                __value.push_str(&detokenize(__val))
                            }
                            _value = Some(__value);
                        }

                        let splited = vars_
                            .split(|pred| pred == &&Token::Coma)
                            .collect::<Vec<_>>();
                        let mut line_descriptors: Vec<LineDescriptions> = vec![LineDescriptions {
                            line: 0,
                            text: "contract{".to_string(),
                        }];

                        let mut positions: Vec<Option<u8>> = Vec::new();

                        for (index, line) in splited.iter().enumerate() {
                            let mut line_text = String::new();
                            if line.is_empty() {
                                positions.push(None);
                            } else {
                                positions.push(Some(index as u8));
                            }
                            for __val in *line {
                                line_text.push_str(&format!("{} ", &detokenize(__val)))
                            }
                            line_descriptors.push(LineDescriptions {
                                text: format!("{};", line_text.trim().to_string()),
                                line: 0,
                            })
                        }
                        line_descriptors.push(LineDescriptions {
                            text: "}".to_string(),
                            line: 0,
                        });

                        let (__variables, _, _, _): (
                            Vec<VariableIdentifier>,
                            Vec<CustomErrorIdentifier>,
                            Vec<MappingIdentifier>,
                            Vec<EventIdentifier>,
                        ) = extract_global_elements(
                            &line_descriptors,
                            custom_data_types,
                            enums,
                            positions,
                            libraries_,
                        );

                        full_block.push(FunctionArm::TuppleAssignment(TuppleAssignment {
                            value: _value.unwrap(),
                            variables: __variables,
                        }))
                    } else {
                        print_error(&format!("Expecting \")\"",))
                    }
                } else if let Token::CloseBraces = _token {
                    // TODO: NOTHING
                } else if let Token::Msg | Token::Payable = _token {
                    let is_payable = if let Token::Payable = _token {
                        true
                    } else {
                        false
                    };
                    extract_low_level_call(block, &mut full_block, is_payable);
                } else {
                    print_error(&format!("Unexpected identifier \"{}\"", detokenize(_token)))
                }
            }
        }
    }
    full_block
}

fn extract_function_call(
    _identifier: &String,
    full_block: &mut Vec<FunctionArm>,
    block: &Vec<&Token>,
) {
    let mut args: Vec<String> = Vec::new();
    let mut variant = String::new();
    let mut has_variant = false;
    let mut variant_index = 0;
    let mut contract_address: Option<String> = None;

    {
        let mut opened_paren_count = 0;
        for (index, blk) in block.iter().enumerate() {
            if let Token::OpenParenthesis = blk {
                opened_paren_count += 1;
            }
            if let Token::CloseParenthesis = blk {
                opened_paren_count -= 1;
            }

            if opened_paren_count == 0 {
                if let Token::Dot = blk {
                    has_variant = true;
                    variant_index = index + 1;
                    break;
                }
            }
        }
    }

    if has_variant {
        {
            if let Token::Dot = block[2] {
                let close_paren_index = &block
                    .iter()
                    .position(|pred| pred == &&Token::CloseParenthesis);
                if let Some(_close_paren_index) = close_paren_index {
                    let address_slice = &block[2..*_close_paren_index];

                    let mut stringified_address = String::new();
                    for __address_slice in address_slice {
                        stringified_address.push_str(&detokenize(&__address_slice));
                    }

                    contract_address = Some(stringified_address);
                }
            } else {
                contract_address = Some(_identifier.to_string());
            }
        }

        if let Token::Identifier(_id) = block[variant_index] {
            variant.push_str(_id);
        } else {
            print_error("Unprocessible variant");
        }
    }

    let tkns = if has_variant {
        block[variant_index + 2..block.len() - 2]
            .split(|pred| pred == &&Token::Coma)
            .collect::<Vec<_>>()
    } else {
        block[2..block.len() - 2]
            .split(|pred| pred == &&Token::Coma)
            .collect::<Vec<_>>()
    };

    for __token_collection in tkns {
        if __token_collection.is_empty() {
            continue;
        }

        let mut stringified = String::new();
        for __token in __token_collection.iter() {
            stringified.push_str(&detokenize(__token));
        }

        if stringified.is_empty() {
            continue;
        }

        args.push(stringified);
    }

    full_block.push(FunctionArm::FunctionCall(FunctionCall {
        arguments: args,
        variant: if has_variant { Some(variant) } else { None },
        identifier: _identifier.to_owned(),
        r#type: if contract_address.is_none() {
            FunctionCallType::Local
        } else {
            FunctionCallType::Contract(contract_address.unwrap())
        },
    }));
}

fn extract_low_level_call(
    block: &Vec<&Token>,
    full_block: &mut Vec<FunctionArm>,
    initial_payable_value: bool,
) {
    let next_variant_index = block.iter().position(|pred| pred == &&Token::Dot);
    if let Some(_index) = next_variant_index {
        let mut address = String::new();
        let _call_variant = &block[.._index];
        let mut call_index = 0;
        let mut payable = initial_payable_value;
        if block.contains(&&Token::Payable) {
            payable = true;
        }
        match _call_variant[0] {
            Token::Msg => {
                address.push_str("msg.sender");
            }
            _ => {
                let _call_position = block
                    .iter()
                    .position(|pred| pred == &&Token::Call || pred == &&Token::Delegatecall);
                call_index = _call_position.unwrap();
                let _slice = &block[.._call_position.unwrap() - 1];

                for __variant in _slice {
                    match __variant {
                        Token::Identifier(_id) => address.push_str(_id),
                        Token::Dot => address.push_str(&detokenize(&__variant)),
                        _ => (),
                    }
                    if let Token::OpenParenthesis | Token::CloseParenthesis = __variant {
                        // TODO: NOTHING
                    } else {
                    }
                }
            }
        }

        let _variants = &block[call_index - 1..];

        match _variants[1] {
            Token::Call | Token::Delegatecall => {
                let mut raw_data: Option<[String; 2]> = None;
                let mut _final = 0;
                if let Token::OpenBraces = _variants[2] {
                    if let Token::Delegatecall = _variants[1] {
                        print_error("Cannot set option \"value\" for delegatecall.")
                    }
                    let close_brace_index = _variants
                        .iter()
                        .position(|pred| pred == &&Token::CloseBraces);
                    if let Some(__index) = close_brace_index {
                        _final = __index;
                        let _raw_data = &_variants[2 + 1..__index];
                        let mut stringified = String::new();

                        for __raw in _raw_data {
                            match __raw {
                                Token::Identifier(___identifier) => {
                                    stringified.push_str(&___identifier);
                                }
                                _other => {
                                    if SYMBOLS.contains(&detokenize(&_other).as_str()) {
                                        stringified.push_str(&detokenize(&_other));
                                    } else {
                                        print_error("Expecting identifier")
                                    }
                                }
                            }
                        }
                        let split = stringified.split(":").collect::<Vec<_>>();
                        if split.len() != 2 {
                            print_error("Unprocessible Entity");
                        } else {
                            if split[0] != "value" {
                                print_error("Expecting \"value\"");
                            } else {
                                raw_data = Some([split[0].to_owned(), split[1].to_owned()]);
                            }
                        }
                    } else {
                        print_error("Unprocessible entity");
                    }
                }

                let __args_variants = if _final == 0 {
                    &_variants[3.._variants.len() - 2]
                } else {
                    &_variants[_final + 2.._variants.len() - 2]
                };

                if __args_variants.is_empty() {
                    print_error("Expecting args for \"call\"");
                }

                let main_args = if let Token::Quotation = __args_variants[0] {
                    &__args_variants[1..__args_variants.len() - 1]
                } else {
                    __args_variants
                };
                let mut __stringified = String::new();

                for _main_arg in main_args {
                    __stringified.push_str(&detokenize(&_main_arg));
                }
                let structured = CallIdentifier {
                    address,
                    payable,
                    arguments: vec![__stringified],
                    raw_data,
                    r#type: if let Token::Delegatecall = _variants[1] {
                        CallIdentifierType::Delegatecall
                    } else {
                        CallIdentifierType::Call
                    },
                };
                full_block.push(FunctionArm::CallIdentifier(structured));
            }
            _ => print_error("Unprocessible entity"),
        }
    } else {
        print_error("Unprocessible entity");
    }
}

fn extract_function_variable(
    block: &Vec<&Token>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    libraries_: &Vec<LibraryIdentifier>,
) -> Option<VariableIdentifier> {
    let mut text = String::new();
    for strr in block {
        text.push_str(&format!("{} ", &detokenize(strr)))
    }

    let variable = validate_variable(
        LineDescriptions { text, line: 0 },
        custom_data_types,
        enums,
        true,
        None,
        libraries_,
    );
    variable.0
}

fn extract_function_scope_variable(
    var: Option<&VariableIdentifier>,
    mappings: Option<&MappingIdentifier>,
    block: &Vec<&Token>,
    _identifier: &String,
) -> Option<FunctionArm> {
    if let Some(_var) = var {
        if _var.is_array {
            if let Token::Dot = block[1] {
                if let Some(_size) = &_var.size {
                    print_error(&format!(
                        "Cannot call a method on a fixed size array \"{}\"",
                        _identifier
                    ))
                }
                let mut value = String::new();
                for val in &block[4..block.len() - 2] {
                    value.push_str(&detokenize(val))
                }

                if let Token::Pop = block[2] {
                    if !value.trim().is_empty() {
                        print_error(&format!("Pop method cannot be assigned value"));
                    }
                }

                return Some(FunctionArm::VariableAssign(VariableAssign {
                    identifier: _identifier.to_string(),
                    operation: if let Token::Push = block[2] {
                        VariableAssignOperation::Push
                    } else {
                        VariableAssignOperation::Pop
                    },
                    variants: None,
                    type_: VariableAssignType::Array(None),
                    value,
                }));
            } else if let Token::OpenSquareBracket = block[1] {
                let equals_index = block.iter().position(|pred| pred == &&Token::Equals);
                if let Some(_equals_index) = equals_index {
                    let close_bracket_position = block[2.._equals_index]
                        .iter()
                        .position(|pred| pred == &&Token::CloseSquareBracket);

                    if let Some(_close_bracket_position) = close_bracket_position {
                        let mut stringified_index = String::new();
                        let index = &block[2.._equals_index][.._close_bracket_position];
                        if index.is_empty() {
                            print_error("Cannot set empty index");
                        }
                        for idx in index {
                            stringified_index.push_str(&detokenize(idx));
                        }
                        let values = &block[_equals_index + 1..block.len() - 1];
                        let mut value = String::new();
                        for val in values {
                            value.push_str(&detokenize(val));
                        }

                        return Some(FunctionArm::VariableAssign(VariableAssign {
                            identifier: _identifier.to_string(),
                            operation: VariableAssignOperation::Assign,
                            variants: None,
                            type_: VariableAssignType::Array(Some(stringified_index)),
                            value,
                        }));
                    } else {
                        print_error(&format!("Unprocessible Entity {}", _identifier))
                    }
                } else {
                    print_error(&format!("Unprocessible Entity {}", _identifier))
                }
            } else {
                print_error(&format!("Unprocessible Entity {}", _identifier))
            }
            None
        } else {
            let equals_index = block.iter().position(|pred| pred == &&Token::Equals);
            if let Some(_position) = equals_index {
                let mut value = String::new();
                let mut stringified = String::new();
                for val in &block[1..block.len() - 1] {
                    stringified.push_str(&detokenize(val));
                }
                if stringified.starts_with("+=") {
                    let other_val_index = stringified.find("=");
                    if let Some(_index) = other_val_index {
                        value = format!("{}+{}", _identifier, &stringified[_index + 1..]);
                        Some(FunctionArm::VariableAssign(VariableAssign {
                            identifier: _identifier.to_string(),
                            operation: VariableAssignOperation::Assign,
                            variants: None,
                            type_: VariableAssignType::Expression,
                            value,
                        }))
                    } else {
                        print_error(&format!("Missing value identifier {}", stringified));
                        process::exit(1);
                    }
                } else if stringified.starts_with("-=") {
                    let other_val_index = stringified.find("=");
                    if let Some(_index) = other_val_index {
                        value = format!("{}-{}", _identifier, &stringified[_index + 1..]);
                        Some(FunctionArm::VariableAssign(VariableAssign {
                            identifier: _identifier.to_string(),
                            operation: VariableAssignOperation::Assign,
                            variants: None,
                            type_: VariableAssignType::Expression,
                            value,
                        }))
                    } else {
                        print_error(&format!("Missing value identifier {}", stringified));
                        process::exit(1);
                    }
                } else {
                    for val in &block[_position + 1..block.len() - 1] {
                        value.push_str(&detokenize(val));
                    }
                    if let VariableType::Enum = _var.type_ {
                        Some(FunctionArm::VariableAssign(VariableAssign {
                            identifier: _identifier.to_string(),
                            operation: VariableAssignOperation::Assign,
                            variants: None,
                            type_: VariableAssignType::Enum,
                            value,
                        }))
                    } else if let VariableType::Struct = _var.type_ {
                        if let Token::Dot = block[1] {
                            Some(FunctionArm::VariableAssign(VariableAssign {
                                identifier: _identifier.to_string(),
                                operation: VariableAssignOperation::Assign,
                                variants: Some(vec![detokenize(block[2])]),
                                type_: VariableAssignType::Struct,
                                value,
                            }))
                        } else {
                            Some(FunctionArm::VariableAssign(VariableAssign {
                                identifier: _identifier.to_string(),
                                operation: VariableAssignOperation::Assign,
                                variants: None,
                                type_: VariableAssignType::Struct,
                                value,
                            }))
                        }
                    } else {
                        Some(FunctionArm::VariableAssign(VariableAssign {
                            identifier: _identifier.to_string(),
                            operation: VariableAssignOperation::Assign,
                            variants: None,
                            type_: VariableAssignType::Expression,
                            value,
                        }))
                    }
                }
            } else if let Some(_) = extract_integer_types_from_token(&_var.data_type) {
                let mut stringified = String::new();
                let mut value = String::new();
                for val in &block[1..block.len() - 1] {
                    stringified.push_str(&detokenize(val));
                }
                if stringified == "++" {
                    value = format!("{}+1", _identifier)
                } else if stringified == "--" {
                    value = format!("{}-1", _identifier)
                } else {
                    print_error(&format!("Unprocessible entity {}", stringified));
                }

                Some(FunctionArm::VariableAssign(VariableAssign {
                    identifier: _identifier.to_string(),
                    operation: VariableAssignOperation::Assign,
                    variants: None,
                    type_: VariableAssignType::Expression,
                    value,
                }))
            } else {
                if block.contains(&&Token::Push) {
                    let value_start_index = block.iter().position(|pred| pred == &&Token::Push);
                    let mut value = String::new();

                    for val_preset in &block[value_start_index.unwrap() + 2..block.len() - 2] {
                        value.push_str(&detokenize(&val_preset));
                    }

                    let first_dot = block.iter().position(|pred| pred == &&Token::Dot);
                    let mut variants: Vec<String> = Vec::new();
                    let variants_preset =
                        &block[first_dot.unwrap() + 1..value_start_index.unwrap()];

                    for _variant_preset in variants_preset {
                        match _variant_preset {
                            Token::Identifier(_id) => variants.push(_id.to_string()),
                            Token::Dot => (),
                            _ => {
                                print_error("Unprocessible Entity");
                            }
                        }
                    }
                    return Some(FunctionArm::VariableAssign(VariableAssign {
                        identifier: _identifier.to_string(),
                        operation: if block.contains(&&Token::Push) {
                            VariableAssignOperation::Push
                        } else {
                            VariableAssignOperation::Pop
                        },
                        variants: if variants.is_empty() {
                            None
                        } else {
                            Some(variants)
                        },
                        type_: VariableAssignType::Struct,
                        value,
                    }));
                } else {
                    print_error(&format!("Missing = {:?}", block));
                }

                None
            }
        }
    } else if let Some(_var) = mappings {
        let equals_index = block.iter().position(|pred| pred == &&Token::Equals);
        let check_open_square_bracket = block
            .iter()
            .find(|pred| pred == &&&Token::OpenSquareBracket);
        if check_open_square_bracket.is_none() {
            print_error(&format!(
                "Missing key for mapping assignment. \"{}\"",
                _identifier
            ));
        }
        if let Some(_position) = equals_index {
            let mut value = String::new();
            let mut stringified = String::new();
            for val in &block[1..block.len() - 1] {
                stringified.push_str(&detokenize(val));
            }

            let variants = extract_mapping_variants(_position, block);
            if stringified.contains("+=") {
                let other_val_index = stringified.find("=");
                if let Some(_index) = other_val_index {
                    value = format!("{}+{}", _identifier, &stringified[_index + 1..]);

                    return Some(FunctionArm::MappingAssign(MappingAssign {
                        identifier: _identifier.to_string(),
                        operation: VariableAssignOperation::Assign,
                        variants,
                        type_: VariableAssignType::Mapping,
                        value,
                    }));
                } else {
                    print_error(&format!("Missing value identifier {}", stringified));
                    process::exit(1);
                }
            } else if stringified.contains("-=") {
                let other_val_index = stringified.find("=");
                if let Some(_index) = other_val_index {
                    value = format!("{}-{}", _identifier, &stringified[_index + 1..]);
                    return Some(FunctionArm::MappingAssign(MappingAssign {
                        identifier: _identifier.to_string(),
                        operation: VariableAssignOperation::Assign,
                        variants,
                        type_: VariableAssignType::Mapping,
                        value,
                    }));
                } else {
                    print_error(&format!("Missing value identifier {}", stringified));
                    process::exit(1);
                }
            } else {
                for val in &block[_position + 1..block.len() - 1] {
                    value.push_str(&detokenize(val));
                }
                return Some(FunctionArm::MappingAssign(MappingAssign {
                    identifier: _identifier.to_string(),
                    operation: VariableAssignOperation::Assign,
                    variants,
                    type_: VariableAssignType::Mapping,
                    value,
                }));
            }
        } else {
            let mut stringified = String::new();
            let mut value = String::new();
            let mut operation = VariableAssignOperation::Assign;
            let mut _open_square_bracket = 1;
            let check_open_square_bracket = block
                .iter()
                .find(|pred| pred == &&&Token::OpenSquareBracket);
            if check_open_square_bracket.is_none() {
                print_error(&format!(
                    "Missing key for mapping assignment. \"{}\"",
                    _identifier
                ));
            }
            for (index, __brac) in block[2..].iter().enumerate() {
                if let Token::CloseSquareBracket = __brac {
                    let next_val = &block[2..].get(index + 1);
                    if let Some(_next) = next_val {
                        if let Token::OpenSquareBracket = _next {
                        } else {
                            break;
                        }
                    }
                }
                _open_square_bracket += 1;
            }

            let variants = extract_mapping_variants(_open_square_bracket + 2, block);

            for val in &block[1..block.len() - 1] {
                stringified.push_str(&detokenize(val));
            }
            if stringified == "++" {
                value = format!("{}+1", _identifier)
            } else if stringified == "--" {
                value = format!("{}-1", _identifier)
            } else if stringified.contains("push") || stringified.contains("pop") {
                let map = mappings.iter().find(|pred| &pred.name == _identifier);
                if let Some(_ret) = map {
                    let map_return = _ret.map.get_return_type();

                    if let Some(_return) = map_return {
                        if _return.contains("[") {
                            let _open_bracket_index = _return.find("[");
                            let _close_bracket_index = _return.find("]");
                            if let Some(_close) = _close_bracket_index {
                                if _close - _open_bracket_index.unwrap() > 1 {
                                    print_error(&format!(
                                        "Cannot call a method on a fixed size array \"{_identifier}\""
                                    ))
                                }
                            } else {
                                print_error("Unprocessible entity");
                            }
                        }
                    } else {
                        print_error("Unprocessible entity");
                    }
                } else {
                    print_error(&format!("Undefined variable \"{_identifier}\""));
                }
                if stringified.contains("push") {
                    operation = VariableAssignOperation::Push;
                } else {
                    operation = VariableAssignOperation::Pop;
                }
                let _open_bracket_index = stringified.find("(");
                if let Some(_index) = _open_bracket_index {
                    let _close_bracket_index = stringified.find(")");
                    if _close_bracket_index.is_none() {
                        print_error(&format!("Unprocessible entity {}", stringified));
                    }
                    let val = &stringified[_index + 1.._close_bracket_index.unwrap()];
                    if let VariableAssignOperation::Pop = operation {
                        if !val.trim().is_empty() {
                            print_error(&format!("Pop method cannot be assigned value"));
                        }
                    }
                    value = val.to_string();
                }
            } else {
                print_error(&format!("Unprocessible entity {}", stringified));
            }

            return Some(FunctionArm::MappingAssign(MappingAssign {
                identifier: _identifier.to_string(),
                operation,
                variants,
                type_: VariableAssignType::Mapping,
                value,
            }));
        }
    } else {
        None
    }
}

fn extract_mapping_variants(_position: usize, block: &Vec<&Token>) -> Vec<String> {
    let mut variants: Vec<String> = Vec::new();
    let mut combo = String::new();
    let mut opened_brackets = 0;
    for __variant in &block[1.._position] {
        if let Token::CloseSquareBracket = __variant {
            opened_brackets -= 1;
            variants.push(combo.clone());
            combo.clear();
        } else if let Token::OpenSquareBracket = __variant {
            if opened_brackets > 0 {
                combo.push_str(&detokenize(&__variant));
            } else {
                opened_brackets += 1;
            }
        } else {
            combo.push_str(&detokenize(&__variant));
        }
    }

    variants
}
