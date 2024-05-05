use crate::mods::{
    constants::constants::SYMBOLS,
    functions::helpers::helpers::{print_error, validate_variable},
    types::types::{LineDescriptions, MappingIdentifier, OpenedBraceType, VariableIdentifier},
};

pub fn extract_global_elements(
    data: &Vec<LineDescriptions>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    variable_positions: Vec<Option<u8>>,
) -> (
    Vec<VariableIdentifier>,
    Vec<String>,
    Vec<MappingIdentifier>,
    Vec<String>,
) {
    let mut global_variables: Vec<VariableIdentifier> = Vec::new();
    let mut custom_errors: Vec<String> = Vec::new();
    let mut events: Vec<String> = Vec::new();
    let mut mappings: Vec<MappingIdentifier> = Vec::new();

    let mut opened_braces = 0;
    let mut opened_brace_type = OpenedBraceType::None;
    let mut variables: Vec<LineDescriptions> = Vec::new();
    let mut combo = String::new();
    for sst in data {
        if sst.text.starts_with("contract") {
            opened_brace_type = OpenedBraceType::Contract;
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
            if let OpenedBraceType::Contract = opened_brace_type {
                if !sst.text.contains("fallback")
                    && !sst.text.contains("receive")
                    && !sst.text.contains("cron")
                    && !sst.text.contains("function")
                    && !sst.text.contains("modifier")
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
            custom_errors.push(_custom_err)
        } else if let Some(_mapping) = validated.2 {
            mappings.push(_mapping)
        } else if let Some(_event) = validated.3 {
            events.push(_event)
        }
    }

    (global_variables, custom_errors, mappings, events)
}
