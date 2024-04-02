use std::process;

use crate::mods::{
    constants::constants::DATA_TYPES,
    functions::{
        controllers::process_state_variables::extract_global_variables,
        helpers::helpers::{
            detokenize, extract_data_location_from_token, extract_integer_types_from_token,
            print_error, validate_expression, validate_identifier_regex, validate_variable,
        },
    },
    types::types::{
        Argument, ConditionalType, Conditionals, Delete, ElIf, FunctionArm, FunctionArmType,
        FunctionCall, FunctionIdentifier, FunctionMutability, LineDescriptions, Loop, LoopType,
        MappingIdentifier, OpenedBraceType, Require, Return, ReturnType, Token, TuppleAssignment,
        VariableAssign, VariableAssignOperation, VariableAssignType, VariableIdentifier,
        VariableType,
    },
};

pub fn extract_functions(
    data: &Vec<LineDescriptions>,
    custom_data_types: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    enums: &Vec<&str>,
) -> Vec<FunctionIdentifier> {
    let mut opened_braces = 0;
    let mut opened_braces_type = OpenedBraceType::None;
    let mut processed_data: Vec<Vec<LineDescriptions>> = Vec::new();
    let mut combined = Vec::new();
    let mut function_identifiers: Vec<FunctionIdentifier> = Vec::new();
    for line in data {
        let raw = &line.text;

        if raw.contains("{") {
            for character in raw.chars() {
                if character == '{' {
                    opened_braces += 1;
                }
            }
        }

        if raw.contains("}") {
            for character in raw.chars() {
                if character == '}' {
                    opened_braces -= 1;
                    if opened_braces == 1 {
                        if let OpenedBraceType::Function = opened_braces_type {
                            opened_braces_type = OpenedBraceType::Contract;
                            combined.push(line.clone());

                            processed_data.push(combined.clone());
                            combined.clear();
                        }
                    }
                }
            }
        }

        if raw.starts_with("function") {
            opened_braces_type = OpenedBraceType::Function;
        }

        if let OpenedBraceType::Function = opened_braces_type {
            combined.push(line.clone())
        }
    }

    let mut stringified = Vec::new();

    for processed in processed_data {
        let mut combined = String::new();
        for prr in processed {
            combined.push_str(&prr.text);
        }

        stringified.push(combined.clone());
        combined.clear();
    }

    for single_stringified in stringified {
        let tokens = LineDescriptions::to_token(single_stringified.as_str());
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
        let function_name = match &tokens[1] {
            Token::Identifier(_val) => {
                let validated = validate_identifier_regex(_val, 0);
                if validated {
                    _val
                } else {
                    process::exit(1)
                }
            }
            _ => {
                print_error(&format!(
                    "Unsupported function name \"{}\"",
                    LineDescriptions::from_token_to_string(&tokens[1])
                ));
                process::exit(1);
            }
        };
        let mut function_override: bool = false;
        let mut function_virtual: bool = false;
        let mut gasless: bool = false;
        let mut function_mutability = FunctionMutability::Mutable;
        let mut function_visibility = Token::Internal;
        let mut function_returns: Option<Vec<ReturnType>> = None;
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
        let params_block =
            &function_definition[start_params.unwrap() + 1..start_params.unwrap() + pad];
        let splited_params_block: Vec<&[Token]> =
            params_block.split(|pred| pred == &Token::Coma).collect();

        let function_arguments =
            extract_function_params(splited_params_block, function_definition, custom_data_types);
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
            ));
        }

        let function_body_start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
        if let None = function_body_start_index {
            print_error(&format!("Unprocessible entity",));
        }

        let function_body = &tokens[function_body_start_index.unwrap()..];
        // println!("{:?}", function_body);
        let arms: Vec<FunctionArm> = extract_function_arms(
            &function_body.to_vec(),
            custom_data_types,
            global_variables,
            enums,
            Vec::new(),
        );

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
            gasless = true;
        }
        let structure: FunctionIdentifier = FunctionIdentifier {
            arguments: function_arguments,
            arms,
            mutability: function_mutability,
            gasless,
            name: function_name.to_string(),
            r#override: function_override,
            returns: function_returns,
            r#virtual: function_virtual,
            visibility: function_visibility,
        };
        function_identifiers.push(structure);
    }

    function_identifiers
}

fn extract_function_params(
    splited_params_block: Vec<&[Token]>,
    function_definition: &[Token],
    custom_data_types: &Vec<&str>,
) -> Vec<Argument> {
    let mut function_arguments: Vec<Argument> = Vec::new();

    for splited_param in splited_params_block {
        if !splited_param.is_empty() {
            let mut type_: Option<String> = None;
            let mut name_: Option<String> = None;
            let mut location_: Option<Token> = None;
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
                    || custom_data_types.contains(
                        &LineDescriptions::from_token_to_string(&splited_param[0]).as_str(),
                    )
                {
                    if let Token::String = splited_param[0] {
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

            if is_array {
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
                if splited_param.contains(&Token::Memory)
                    && splited_param.contains(&Token::Calldata)
                {
                    print_error(&format!(
                        "Expecting \"memory\" or \"calldata\". {} ",
                        vec_.join(" "),
                    ))
                }
            }

            let structured = Argument {
                location: location_,
                name_: name_.unwrap(),
                type_: type_.unwrap(),
                is_array,
                size,
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
                if let Token::String = splited_param[0] {
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
                    is_primitive = false;
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
    custom_data_types: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    enums: &Vec<&str>,
    local_vars: Vec<&VariableIdentifier>,
) -> Vec<FunctionArm> {
    let mut arms: Vec<Vec<&Token>> = Vec::new();
    let mut combined: Vec<&Token> = Vec::new();
    // println!("{:?}", body);
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

            _ => (),
        }
    }

    let mut joined_conditionals: Vec<Vec<&Token>> = Vec::new();

    for arm in arms {
        if let Token::Else = &arm[0] {
            let last_index = joined_conditionals.len() - 1;
            for sec in arm {
                joined_conditionals[last_index].push(sec);
            }
        } else {
            joined_conditionals.push(arm.to_owned());
        }
    }
    // println!("{:#?} \n\n\n\n\n", joined_conditionals);

    extract_function_block(
        &joined_conditionals,
        custom_data_types,
        enums,
        global_variables,
        local_vars,
    )
}

fn extract_function_block(
    arms: &Vec<Vec<&Token>>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
    local_vars: Vec<&VariableIdentifier>,
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
                    let variable = extract_function_variable(&block, custom_data_types, enums);
                    if let None = variable {
                        print_error("OOPS!!!");
                    }

                    full_block.push(FunctionArm::VariableIdentifier(variable.unwrap()));
                } else {
                    if let Token::OpenParenthesis = &block[1] {
                        let mut args: Vec<String> = Vec::new();

                        for arg in &block[2..block.len() - 2] {
                            match arg {
                                Token::Identifier(_id) => {
                                    args.push(_id.to_string());
                                }
                                Token::Coma => (),
                                _ => print_error(&format!("Invalid function call")),
                            }
                        }

                        full_block.push(FunctionArm::FunctionCall(FunctionCall {
                            arguments: args,
                            identifier: _identifier.to_owned(),
                        }));
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
                                let function_scope_variable =
                                    extract_function_scope_variable(_var, block, _identifier);
                                if let Some(_) = function_scope_variable {
                                    full_block.push(function_scope_variable.unwrap())
                                }
                            } else {
                                print_error(&format!("Unidentified variable {}", _identifier))
                            }
                        } else {
                            let global_variables_identifiers: Vec<&String> =
                                global_variables.iter().map(|pred| &pred.name).collect();

                            if global_variables_identifiers.contains(&_identifier) {
                                let var = global_variables
                                    .iter()
                                    .find(|pred| pred.name == _identifier.to_string());

                                if let Some(_var) = var {
                                    let function_scope_variable =
                                        extract_function_scope_variable(_var, block, _identifier);
                                    if let Some(_) = function_scope_variable {
                                        full_block.push(function_scope_variable.unwrap());
                                    }
                                } else {
                                    print_error(&format!("Unidentified variable {}", _identifier))
                                }
                            } else {
                                print_error(&format!("Unidentified variable {}", _identifier))
                            }
                        }
                    }
                }
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
                        batched.push(_batch.clone().clone());
                    }
                    let mut local_vars: Vec<&VariableIdentifier> = Vec::new();
                    for __blk in &full_block {
                        if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                            local_vars.push(_identifier)
                        }
                    }

                    let __arms = extract_function_arms(
                        &batched,
                        custom_data_types,
                        global_variables,
                        enums,
                        local_vars,
                    );

                    let structured = FunctionArm::Loop(Loop {
                        arms: __arms,
                        condition: _condition,
                        identifier: None,
                        op: None,
                        value: None,
                        r#type: LoopType::While,
                    });

                    // panic!("{:#?}", structured);
                    full_block.push(structured);
                }
                // panic!("{:?}", block);
            }
            Token::If => {
                let mut conditional_index = 0;
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
                                    //
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
                                    condition.push(blk.clone().clone());
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
                        batched.push(blk.clone().clone());
                    }

                    if opened_braces == 0 && index > 0 {
                        match conditional_type {
                            ConditionalType::If => {
                                if !batched.is_empty() {
                                    batched.push(Token::CloseBraces);
                                }
                                let mut local_vars: Vec<&VariableIdentifier> = Vec::new();
                                for __blk in &full_block {
                                    if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                                        local_vars.push(_identifier)
                                    }
                                }
                                let __arm: Vec<FunctionArm> = extract_function_arms(
                                    &batched,
                                    custom_data_types,
                                    global_variables,
                                    enums,
                                    local_vars,
                                );
                                conditional_index += 1;
                                tree.arm = __arm;

                                batched.clear();
                            }

                            ConditionalType::ElIf => {
                                if !batched.is_empty() {
                                    batched.push(Token::CloseBraces);
                                }
                                let mut local_vars: Vec<&VariableIdentifier> = Vec::new();
                                for __blk in &full_block {
                                    if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                                        local_vars.push(_identifier)
                                    }
                                }
                                let __arm: Vec<FunctionArm> = extract_function_arms(
                                    &batched,
                                    custom_data_types,
                                    global_variables,
                                    enums,
                                    local_vars,
                                );

                                let last_len = tree.elif.len();
                                if last_len > 0 {
                                    tree.elif[last_len - 1].arm = __arm;
                                }
                                batched.clear();
                            }

                            ConditionalType::El => {
                                if !batched.is_empty() {
                                    batched.push(Token::CloseBraces);
                                }

                                let mut local_vars: Vec<&VariableIdentifier> = Vec::new();
                                for __blk in &full_block {
                                    if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                                        local_vars.push(_identifier)
                                    }
                                }

                                let __arm: Vec<FunctionArm> = extract_function_arms(
                                    &batched,
                                    custom_data_types,
                                    global_variables,
                                    enums,
                                    local_vars,
                                );
                                // println!("{:?}", batched);
                                tree.el = Some(__arm);
                                batched.clear();

                                // println!("sdfasd here {:?}", batched);
                            }
                            _ => (),
                        }
                    }
                }

                if conditional_index > 0 {
                    conditional_index -= 1;
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
                                    variant: None,
                                    data_type: _var.data_type.clone(),
                                }))
                            } else if let VariableType::Struct = _var.type_ {
                                if let Token::Dot = block[2] {
                                    let mut variants = String::new();
                                    for _variant in &block[3..block.len() - 1] {
                                        variants.push_str(&detokenize(_variant))
                                    }
                                    full_block.push(FunctionArm::Delete(Delete {
                                        identifier: _identifier.to_string(),
                                        type_: VariableAssignType::Struct,
                                        variant: Some(variants),
                                        data_type: _var.data_type.clone(),
                                    }));
                                } else {
                                    full_block.push(FunctionArm::Delete(Delete {
                                        identifier: _identifier.to_string(),
                                        type_: VariableAssignType::Struct,
                                        variant: None,
                                        data_type: _var.data_type.clone(),
                                    }));
                                }
                            } else {
                                full_block.push(FunctionArm::Delete(Delete {
                                    identifier: _identifier.to_string(),
                                    type_: VariableAssignType::Expression,
                                    variant: None,
                                    data_type: _var.data_type.clone(),
                                }))
                            }
                        } else {
                            print_error(&format!("Unidentified variable {}", _identifier))
                        }
                    } else {
                        let global_variables_identifiers: Vec<&String> =
                            global_variables.iter().map(|pred| &pred.name).collect();

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
                                        variant: None,
                                        data_type: _var.data_type.clone(),
                                    }))
                                } else if let VariableType::Struct = _var.type_ {
                                    if let Token::Dot = block[2] {
                                        let mut variants = String::new();
                                        for _variant in &block[3..block.len() - 1] {
                                            variants.push_str(&detokenize(_variant))
                                        }
                                        full_block.push(FunctionArm::Delete(Delete {
                                            identifier: _identifier.to_string(),
                                            type_: VariableAssignType::Struct,
                                            variant: Some(variants),
                                            data_type: _var.data_type.clone(),
                                        }));
                                    } else {
                                        full_block.push(FunctionArm::Delete(Delete {
                                            identifier: _identifier.to_string(),
                                            type_: VariableAssignType::Struct,
                                            variant: None,
                                            data_type: _var.data_type.clone(),
                                        }));
                                    }
                                } else {
                                    full_block.push(FunctionArm::Delete(Delete {
                                        identifier: _identifier.to_string(),
                                        type_: VariableAssignType::Expression,
                                        variant: None,
                                        data_type: _var.data_type.clone(),
                                    }))
                                }
                            } else {
                                print_error(&format!("Unidentified variable {}", _identifier))
                            }
                        } else {
                            print_error(&format!("Unidentified variable {}", _identifier))
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
                let mut _operation = String::new();
                if let Some(_open_brace_index) = open_brace_index {
                    let condition_block = &block[2.._open_brace_index - 1];

                    let splitted = condition_block
                        .split(|pred| pred == &&Token::SemiColon)
                        .collect::<Vec<_>>();

                    if splitted.len() != 3 {
                        print_error("Unprocessible entity for loop");
                    } else {
                        if let Token::Uint | Token::Int = &splitted[0][0] {
                            match splitted[0][1] {
                                Token::Identifier(_id) => __identifier = _id.to_string(),
                                _ => {
                                    print_error("Unprocessible entity for loop");
                                }
                            }

                            if let Token::Equals = splitted[0][2] {
                                //
                            } else {
                                print_error("Identifier needs assignment");
                            }
                            match splitted[0][3] {
                                Token::Identifier(_id) => _value = Some(_id.to_string()),
                                _ => {
                                    print_error("Unprocessible entity for loop");
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
                        _operation.push_str(&detokenize(__op));
                    }

                    // panic!("{:?}", _operation);
                    let mut batched: Vec<Token> = Vec::new();
                    for _batch in &block[_open_brace_index..] {
                        batched.push(_batch.clone().clone());
                    }
                    let mut local_vars: Vec<&VariableIdentifier> = Vec::new();
                    for __blk in &full_block {
                        if let FunctionArm::VariableIdentifier(_identifier) = __blk {
                            local_vars.push(_identifier)
                        }
                    }
                    let __arms = extract_function_arms(
                        &batched,
                        custom_data_types,
                        global_variables,
                        enums,
                        local_vars,
                    );

                    let structured = FunctionArm::Loop(Loop {
                        arms: __arms,
                        condition: _condition,
                        identifier: Some(__identifier),
                        op: Some(_operation),
                        value: _value,
                        r#type: LoopType::For,
                    });
                    full_block.push(structured);
                } else {
                    print_error("Unprocessible Entity for for loop");
                }
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
                    _ => print_error("Missing ;"),
                }
                if DATA_TYPES.contains(&detokenize(_token).as_str()) {
                    let variable = extract_function_variable(&block, custom_data_types, enums);
                    if let None = variable {
                        print_error("OOPS!!!");
                    }

                    full_block.push(FunctionArm::VariableIdentifier(variable.unwrap()));
                } else if let Token::OpenParenthesis = _token {
                    let end_index = block
                        .iter()
                        .position(|pred| pred == &&Token::CloseParenthesis);
                    if let Some(_index) = end_index {
                        let vars_ = &block[1.._index];
                        let mut value: Option<String> = None;

                        {
                            let mut __value = String::new();
                            for __val in &block[_index + 2..block.len() - 1] {
                                __value.push_str(&detokenize(__val))
                            }
                            value = Some(__value);
                        }

                        let splited = vars_
                            .split(|pred| pred == &&Token::Coma)
                            .collect::<Vec<_>>();

                        let mut line_descriptors: Vec<LineDescriptions> = vec![LineDescriptions {
                            line: 0,
                            text: "contract{".to_string(),
                        }];
                        for line in splited {
                            let mut line_text = String::new();
                            for __val in line {
                                line_text.push_str(&format!("{} ", &detokenize(__val)))
                            }
                            line_descriptors.push(LineDescriptions {
                                text: line_text.trim().to_string(),
                                line: 0,
                            })
                        }
                        line_descriptors.push(LineDescriptions {
                            text: "}".to_string(),
                            line: 0,
                        });
                        let (__variables, _, _): (
                            Vec<VariableIdentifier>,
                            Vec<String>,
                            Vec<MappingIdentifier>,
                        ) = extract_global_variables(&line_descriptors, custom_data_types, enums);
                        full_block.push(FunctionArm::TuppleAssignment(TuppleAssignment {
                            value: value.unwrap(),
                            variables: __variables,
                        }))
                        // println!("{:?}", &line_descriptors);
                        // println!("{:?}", __variables)
                    } else {
                        print_error(&format!("Expecting \")\"",))
                    }
                } else {
                    print_error(&format!("Unexpected identifier \"{}\"", detokenize(_token)))
                }
            }
        }
    }
    full_block
}

fn extract_function_variable(
    block: &Vec<&Token>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
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
    );
    variable.0
}

fn extract_function_scope_variable(
    _var: &VariableIdentifier,
    block: &Vec<&Token>,
    _identifier: &String,
) -> Option<FunctionArm> {
    if _var.is_array {
        if let Token::Dot = block[1] {
            if let Some(_size) = &_var.size {
                print_error(&format!(
                    "Cannot push to a fixed size array \"{}\"",
                    _identifier
                ))
            }
            let mut value = String::new();
            for val in &block[4..block.len() - 2] {
                value.push_str(&detokenize(val))
            }

            return Some(FunctionArm::VariableAssign(VariableAssign {
                identifier: _identifier.to_string(),
                operation: if let Token::Push = block[2] {
                    VariableAssignOperation::Push
                } else {
                    VariableAssignOperation::Pop
                },
                variant: None,
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
                        variant: None,
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
            for val in &block[_position + 1..block.len() - 1] {
                value.push_str(&detokenize(val));
            }
            if let VariableType::Enum = _var.type_ {
                Some(FunctionArm::VariableAssign(VariableAssign {
                    identifier: _identifier.to_string(),
                    operation: VariableAssignOperation::Assign,
                    variant: None,
                    type_: VariableAssignType::Enum,
                    value,
                }))
            } else if let VariableType::Struct = _var.type_ {
                if let Token::Dot = block[1] {
                    Some(FunctionArm::VariableAssign(VariableAssign {
                        identifier: _identifier.to_string(),
                        operation: VariableAssignOperation::Assign,
                        variant: Some(detokenize(block[2])),
                        type_: VariableAssignType::Struct,
                        value,
                    }))
                } else {
                    Some(FunctionArm::VariableAssign(VariableAssign {
                        identifier: _identifier.to_string(),
                        operation: VariableAssignOperation::Assign,
                        variant: None,
                        type_: VariableAssignType::Struct,
                        value,
                    }))
                }
            } else {
                Some(FunctionArm::VariableAssign(VariableAssign {
                    identifier: _identifier.to_string(),
                    operation: VariableAssignOperation::Assign,
                    variant: None,
                    type_: VariableAssignType::Expression,
                    value,
                }))
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
            } else if stringified.starts_with("+=") {
                let other_val_index = stringified.find("=");
                if let Some(_index) = other_val_index {
                    value = format!("{}+{}", _identifier, &stringified[_index + 1..])
                } else {
                    print_error(&format!("Missing value identifier {}", stringified));
                }
            } else if stringified.starts_with("-=") {
                let other_val_index = stringified.find("=");
                if let Some(_index) = other_val_index {
                    value = format!("{}-{}", _identifier, &stringified[_index + 1..])
                } else {
                    print_error(&format!("Missing value identifier {}", stringified));
                }
            } else {
                print_error(&format!("Unprocessible entiry {}", stringified));
            }
            Some(FunctionArm::VariableAssign(VariableAssign {
                identifier: _identifier.to_string(),
                operation: VariableAssignOperation::Assign,
                variant: None,
                type_: VariableAssignType::Expression,
                value,
            }))
        } else {
            print_error(&format!("Missing = {:?}", block));
            None
        }
    }
}
