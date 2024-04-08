use crate::mods::{
    constants::constants::SYMBOLS,
    functions::helpers::helpers::validate_variable,
    types::types::{LineDescriptions, MappingIdentifier, OpenedBraceType, VariableIdentifier},
};

pub fn extract_global_variables(
    data: &Vec<LineDescriptions>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
) -> (Vec<VariableIdentifier>, Vec<String>, Vec<MappingIdentifier>) {
    let mut global_variables: Vec<VariableIdentifier> = Vec::new();
    let mut custom_errors: Vec<String> = Vec::new();
    let mut mappings: Vec<MappingIdentifier> = Vec::new();

    let mut opened_braces = 0;
    let mut opened_brace_type = OpenedBraceType::None;
    let mut variables: Vec<LineDescriptions> = Vec::new();
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
        }

        if sst.text.contains("{") {
            for llm in sst.text.chars() {
                if llm == '{' {
                    opened_braces += 1;
                }
            }
        }

        if sst.text.contains("}") {
            if sst.text.contains("{") {
                continue;
            }
            for llm in sst.text.chars() {
                if llm == '}' {
                    opened_braces -= 1;
                    if opened_braces == 1 {
                        opened_brace_type = OpenedBraceType::Contract;
                    }
                }
            }
        }

        if opened_braces == 1 {
            if let OpenedBraceType::Contract = opened_brace_type {
                if !sst.text.contains("fallback")
                    && !sst.text.contains("receive")
                    && !sst.text.contains("cron")
                    && !sst.text.contains("function")
                {
                    if !SYMBOLS.contains(&sst.text.as_str()) {
                        if !sst.text.starts_with("contract") {
                            let splited: Vec<&str> = sst.text.split(";").collect();
                            for spl in splited {
                                if !spl.trim().is_empty() {
                                    variables.push(LineDescriptions {
                                        text: format!("{spl}"),
                                        line: sst.line,
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for variable in variables {
        let validated = validate_variable(variable, custom_data_types, enums, false);
        if let Some(_raw) = validated.0 {
            global_variables.push(_raw);
        } else if let Some(_custom_err) = validated.1 {
            custom_errors.push(_custom_err)
        } else if let Some(_mapping) = validated.2 {
            mappings.push(_mapping)
        }
    }

    (global_variables, custom_errors, mappings)
}
