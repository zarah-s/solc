use crate::{
    mods::types::types::{
        Mapping, MappingIdentifier, MappingValue, OpenedBraceType, VariableIdentifier, VariableType,
    },
    LineDescriptions, Token,
};
use eval::eval;
use regex::Regex;
// use std::process;
pub fn print_error(msg: &str) {
    panic!("ERROR: {}", msg);
    // process::exit(1);
}

pub fn lex_to_token(input: &str) -> Token {
    let token = match input {
        "revert" => Token::Revert,
        "assert" => Token::Assert,
        "bytes" => Token::Bytes,
        "wei" => Token::Wei,
        "ether" => Token::Ether,
        "event" => Token::Event,
        "while" => Token::While,
        "contract" => Token::Contract,
        "mapping" => Token::Mapping,
        "storage" => Token::Storage,
        "delete" => Token::Delete,
        "push" => Token::Push,
        "pop" => Token::Pop,
        "msg" => Token::Msg,
        "is" => Token::Is,
        "require" => Token::Require,
        "constructor" => Token::Constructor,
        "receive" => Token::Receive,
        "internal" => Token::Internal,
        "external" => Token::External,
        "calldata" => Token::Calldata,
        "fallback" => Token::Fallback,
        "cron" => Token::Cron,
        "enum" => Token::Enum,
        "gasless" => Token::Gasless,
        "true" => Token::True,
        "false" => Token::False,
        "address" => Token::Address,
        "error" => Token::Error,
        "private" => Token::Private,
        "struct" => Token::Struct,
        "function" => Token::Function,
        "public" => Token::Public,
        "view" => Token::View,
        "returns" => Token::Returns,
        "pure" => Token::Pure,
        "override" => Token::Override,
        "constant" => Token::Constant,
        "immutable" => Token::Immutable,
        "mutable" => Token::Mutable,
        "new" => Token::New,
        "virtual" => Token::Virtual,
        "return" => Token::Return,
        "payable" => Token::Payable,
        "memory" => Token::Memory,
        "uint" => Token::Uint,
        "int" => Token::Int,
        "int8" => Token::Int8,
        "uint8" => Token::Uint8,
        "uint16" => Token::Uint16,
        "uint32" => Token::Uint32,
        "uint120" => Token::Uint120,
        "uint256" => Token::Uint256,
        "int16" => Token::Int16,
        "int32" => Token::Int32,
        "int120" => Token::Int120,
        "int256" => Token::Int256,
        "string" => Token::String,
        "bool" => Token::Bool,
        "if" => Token::If,
        "else" => Token::Else,
        "for" => Token::For,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "/" => Token::Divide,
        "*" => Token::Multiply,
        "(" => Token::OpenParenthesis,
        ")" => Token::CloseParenthesis,
        "[" => Token::OpenSquareBracket,
        "]" => Token::CloseSquareBracket,
        "{" => Token::OpenBraces,
        "}" => Token::CloseBraces,
        ">" => Token::GreaterThan,
        "<" => Token::LessThan,
        "." => Token::Dot,
        "=" => Token::Equals,
        "!" => Token::Bang,
        "%" => Token::Modulu,
        ";" => Token::SemiColon,
        "'" => Token::Quotation,
        "\"" => Token::Quotation,
        "," => Token::Coma,
        "|" => Token::Pipe,
        "&" => Token::Ampersand,

        _ => Token::Identifier(input.to_string()),
    };
    token
}

pub fn detokenize(input: &Token) -> String {
    let token: String = match input {
        Token::Contract => "contract".to_string(),
        Token::Assert => "assert".to_string(),
        Token::Is => "is".to_string(),
        Token::Event => "event".to_string(),
        Token::Ether => "ether".to_string(),
        Token::Wei => "wei".to_string(),
        Token::Bytes => "bytes".to_string(),
        Token::Revert => "revert".to_string(),
        Token::Storage => "storage".to_string(),
        Token::While => "while".to_string(),
        Token::True => "true".to_string(),
        Token::False => "false".to_string(),
        Token::Push => "push".to_string(),
        Token::Pop => "pop".to_string(),
        Token::Error => "error".to_string(),
        Token::Delete => "delete".to_string(),
        Token::Require => "require".to_string(),
        Token::Mutable => "mutable".to_string(),
        Token::Immutable => "immutable".to_string(),
        Token::Constant => "constant".to_string(),
        Token::Mapping => "mapping".to_string(),
        Token::Msg => "msg".to_string(),
        Token::Constructor => "constructor".to_string(),
        Token::Calldata => "calldata".to_string(),
        Token::Receive => "receive".to_string(),
        Token::Fallback => "fallback".to_string(),
        Token::Cron => "cron".to_string(),
        Token::Enum => "enum".to_string(),
        Token::Virtual => "virtual".to_string(),
        Token::New => "new ".to_string(),
        Token::Override => "override".to_string(),
        Token::Gasless => "gasless".to_string(),
        Token::Address => "address".to_string(),
        Token::Private => "private".to_string(),
        Token::Struct => "struct".to_string(),
        Token::Function => "function".to_string(),
        Token::Public => "public".to_string(),
        Token::View => "view".to_string(),
        Token::Returns => "returns".to_string(),
        Token::Pure => "pure".to_string(),
        Token::Return => "return ".to_string(),
        Token::External => "external".to_string(),
        Token::Internal => "internal".to_string(),
        Token::Payable => "payable".to_string(),
        Token::Memory => "memory".to_string(),
        Token::Uint => "uint".to_string(),
        Token::Int => "int".to_string(),
        Token::Int8 => "int8".to_string(),
        Token::Uint8 => "uint8".to_string(),
        Token::Uint16 => "uint16".to_string(),
        Token::Uint32 => "uint32".to_string(),
        Token::Uint120 => "uint120".to_string(),
        Token::Uint256 => "uint256".to_string(),
        Token::Int16 => "int16".to_string(),
        Token::Int32 => "int32".to_string(),
        Token::Int120 => "int120".to_string(),
        Token::Int256 => "int256".to_string(),
        Token::String => "string".to_string(),
        Token::Bool => "bool".to_string(),
        Token::If => "if".to_string(),
        Token::Else => "else".to_string(),
        Token::For => "for".to_string(),
        Token::Plus => "+".to_string(),
        Token::Minus => "-".to_string(),
        Token::Divide => "/".to_string(),
        Token::Multiply => "*".to_string(),
        Token::OpenParenthesis => "(".to_string(),
        Token::CloseParenthesis => ")".to_string(),
        Token::OpenSquareBracket => "[".to_string(),
        Token::CloseSquareBracket => "]".to_string(),
        Token::OpenBraces => "{".to_string(),
        Token::CloseBraces => "}".to_string(),
        Token::GreaterThan => ">".to_string(),
        Token::LessThan => "<".to_string(),
        Token::Dot => ".".to_string(),
        Token::Equals => "=".to_string(),
        Token::Bang => "!".to_string(),
        Token::Modulu => "%".to_string(),
        Token::SemiColon => ";".to_string(),
        Token::Quotation => "\"".to_string(),
        Token::Coma => ",".to_string(),
        Token::Pipe => "|".to_string(),
        Token::Ampersand => "&".to_string(),
        Token::Identifier(_val) => _val.to_owned(),
    };
    token
}

pub fn extract_data_location_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Memory => Some(Token::Memory),
        Token::Calldata => Some(Token::Calldata),
        _ => None,
    }
}

pub fn extract_integer_types_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Uint => Some(Token::Uint),
        Token::Uint120 => Some(Token::Uint120),
        Token::Uint16 => Some(Token::Uint16),
        Token::Uint256 => Some(Token::Uint256),
        Token::Uint32 => Some(Token::Uint32),
        Token::Uint8 => Some(Token::Uint8),
        Token::Int => Some(Token::Int),
        Token::Int120 => Some(Token::Int120),
        Token::Int16 => Some(Token::Int16),
        Token::Int32 => Some(Token::Int32),
        Token::Int8 => Some(Token::Int8),

        _ => None,
    }
}

pub fn extract_data_types_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Uint => Some(Token::Uint),
        Token::Uint120 => Some(Token::Uint120),
        Token::Uint16 => Some(Token::Uint16),
        Token::Uint256 => Some(Token::Uint256),
        Token::Uint32 => Some(Token::Uint32),
        Token::Uint8 => Some(Token::Uint8),
        Token::Int => Some(Token::Int),
        Token::Int120 => Some(Token::Int120),
        Token::Int16 => Some(Token::Int16),
        Token::Int32 => Some(Token::Int32),
        Token::Int8 => Some(Token::Int8),
        Token::String => Some(Token::String),
        Token::Bool => Some(Token::Bool),
        Token::Address => Some(Token::Address),
        _ => None,
    }
}

pub fn validate_expression(expression: &String, text: LineDescriptions) -> Option<String> {
    if expression.contains("**") {
        let mut tokenized_expression = LineDescriptions::to_token(&format!("{};", expression));
        let mut detokenized_expression = String::new();
        let mut expr = String::new();
        for token in &tokenized_expression {
            detokenized_expression.push_str(&LineDescriptions::from_token_to_string(&token));
        }
        while detokenized_expression.contains("**") {
            let mut start_position: Option<usize> = None;
            for (i, _token) in tokenized_expression.iter().enumerate() {
                if let Token::Multiply = _token {
                    if let Token::Multiply = tokenized_expression[i + 1] {
                        start_position = Some(i);
                        break;
                    }
                }
            }

            if let Some(_val) = start_position {
                let raised_expr = &tokenized_expression[_val - 1.._val + 3];
                let mut num: usize = 0;
                let mut power: usize = 0;
                if let Token::Identifier(_num_identifier) = &raised_expr[0] {
                    match _num_identifier.parse::<usize>() {
                        Err(_error) => {
                            print_error(&format!("{_error} expecting number {},", text.text))
                        }
                        Ok(_value) => {
                            if _value > 0 {
                                num = _value;
                            } else {
                                print_error(&format!("Found 0 {}", text.text))
                            }
                        }
                    }
                } else {
                    print_error(&format!("expecting number {},", text.text))
                }

                if let Token::Identifier(_power_identifier) = &raised_expr[raised_expr.len() - 1] {
                    match _power_identifier.parse::<usize>() {
                        Err(_error) => {
                            print_error(&format!("{_error} expecting number {},", text.text))
                        }
                        Ok(_val) => {
                            power = _val;
                        }
                    }

                    let math = num.pow(power as u32);
                    let left_padding = &tokenized_expression.clone()[.._val - 1];
                    let right_padding = &tokenized_expression.clone()[left_padding.len() + 4..];
                    tokenized_expression = [
                        left_padding,
                        &[Token::Identifier(math.to_string())],
                        right_padding,
                    ]
                    .concat();

                    detokenized_expression.clear();
                    for token in tokenized_expression.clone() {
                        detokenized_expression
                            .push_str(&LineDescriptions::from_token_to_string(&token));
                    }
                } else {
                    print_error(&format!("expecting number {},", text.text))
                }
            } else {
                print_error(&format!("Something went wrong {},", text.text))
            }
        }
        for token in tokenized_expression {
            expr.push_str(&LineDescriptions::from_token_to_string(&token));
        }
        evaluate_expression(&expr, text)
    } else {
        evaluate_expression(expression, text)
    }
}

fn evaluate_expression(expression: &String, text: LineDescriptions) -> Option<String> {
    let mut expr = String::new();
    if expression.ends_with(";") {
        expr.push_str(&expression[..expression.len() - 1]);
    } else {
        expr.push_str(expression)
    }
    let sz = eval(&expr);
    let mut size: Option<String> = None;

    match sz {
        Ok(_val) => {
            if _val.to_string().contains(".") {
                match _val.to_string().parse::<f64>() {
                    Err(_error) => {
                        print_error(&format!("Not a decimal {}", text.text));
                    }
                    Ok(_dec) => {
                        let fractional_part = _dec.fract();

                        if fractional_part == 0.0 {
                            size = Some(_dec.trunc().to_string());
                        } else {
                            print_error(&format!(
                                "Uprocessible entity for array size. {}... size cannot be fraction",
                                text.text
                            ));
                        }
                    }
                }
            } else {
                if _val.to_string() == "0" {
                    print_error(&format!(
                        "Uprocessible entity for array size \"0\". {}... ",
                        text.text
                    ));
                } else {
                    size = Some(_val.to_string());
                }
            }
        }
        Err(_err) => print_error(&format!("{_err}. {} ", text.text)),
    }

    if let None = size {
        print_error(&format!(
            "Uprocessible entity for array size {}... ",
            text.text
        ));
    }

    size
}

pub fn validate_identifier_regex(identifer: &str, line: i32) -> bool {
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
    if identifer.is_empty() {
        print_error(&format!(
            "Expecting identifier \"{}\" on line {}",
            identifer, line
        ));
        false
    } else {
        if let Some(_) = Regex::new(r"[\W]").unwrap().find(identifer) {
            print_error(&format!(
                "Invalid Identifier \"{}\" on line {}",
                identifer, line
            ));
            false
        } else {
            if let Some(_id) = identifier_regex.find(identifer) {
                true
            } else {
                print_error(&format!(
                    "Invalid Identifier \"{}\" on line {}",
                    identifer, line
                ));
                false
            }
        }
    }
}

pub fn extract_custom_data_types_tokens(
    _type: &Token,
    data: &Vec<LineDescriptions>,
) -> Vec<Vec<Token>> {
    let mut stringified = String::new();
    let mut extracted_types: Vec<Vec<Token>> = Vec::new();
    let mut combined: Vec<Token> = Vec::new();
    for line_data in data {
        stringified.push_str(&line_data.text);
    }
    let tokens = LineDescriptions::to_token(&stringified);

    let mut opened_braces = 0;
    let mut opened_brace_type = OpenedBraceType::None;

    for token in tokens {
        match token {
            Token::OpenBraces => {
                opened_braces += 1;

                if let Token::Struct = _type {
                    if let OpenedBraceType::Struct = opened_brace_type {
                        combined.push(token)
                    }
                } else {
                    if let OpenedBraceType::Enum = opened_brace_type {
                        combined.push(token)
                    }
                }
            }

            Token::CloseBraces => {
                if let Token::Struct = _type {
                    if let OpenedBraceType::Struct = opened_brace_type {
                        combined.push(token)
                    }
                } else {
                    if let OpenedBraceType::Enum = opened_brace_type {
                        combined.push(token)
                    }
                }
                opened_braces -= 1;
                if opened_braces == 1 {
                    opened_brace_type = OpenedBraceType::None;
                    if !combined.is_empty() {
                        extracted_types.push(combined.clone());
                        combined.clear();
                    }
                }
            }
            Token::Struct => {
                if let Token::Struct = _type {
                    opened_brace_type = OpenedBraceType::Struct;
                    combined.push(token)
                } else {
                }
            }

            Token::Enum => {
                if let Token::Struct = _type {
                } else {
                    opened_brace_type = OpenedBraceType::Enum;
                    combined.push(token)
                }
            }

            _other => {
                if let Token::Struct = _type {
                    if let OpenedBraceType::Struct = opened_brace_type {
                        combined.push(_other)
                    }
                } else {
                    if let OpenedBraceType::Enum = opened_brace_type {
                        combined.push(_other)
                    }
                }
            }
        }
    }

    extracted_types
}

pub fn validate_variable(
    text: LineDescriptions,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    is_function_variable: bool,
) -> (
    Option<VariableIdentifier>,
    Option<String>,
    Option<MappingIdentifier>,
    Option<String>,
) {
    let mut is_array = false;
    let mut size: Option<String> = None;
    let mut data_type: Option<Token> = None;
    let mut is_custom_error = false;
    let mut is_event = false;
    let mut variable_name: Option<String> = None;
    let mut visibility = Token::Internal;
    let mut mutability = Token::Mutable;
    let mut value: Option<String> = None;
    let mut type_ = VariableType::Variable;
    let mut storage: Option<Token> = None;
    let mut is_primitive = true;
    let tokens = LineDescriptions::to_token(&format!("{}", text.text));
    let mut mapping = Mapping::new();
    if let Token::Mapping = &tokens[0] {
        let mut pad = 0;

        for n in 0..tokens.len() {
            if pad > n {
                continue;
            }

            if let Some(_token) = extract_data_types_from_token(&tokens[n]) {
                let next = tokens.get(n + 1);
                if let Some(_next) = next {
                    pad = n + 1;
                    if let Token::Equals = _next {
                        if let Some(_accross_to_value) = tokens.get(n + 3) {
                            pad = n + 3;

                            if let Token::Mapping = _accross_to_value {
                                mapping.insert(Some(detokenize(&tokens[n + 5])), None);
                                pad = n + 5;
                            } else {
                                let _end = &tokens[pad..]
                                    .iter()
                                    .position(|pred| pred == &Token::CloseParenthesis);
                                if _end.is_none() {
                                    print_error("Unprocessible entity on mapping");
                                }

                                let mut combo = String::new();
                                for _token in &tokens[pad..pad + _end.unwrap()] {
                                    combo.push_str(&detokenize(_token))
                                }
                                mapping.insert(None, Some(MappingValue::Raw(combo)))
                            }
                        }
                    }
                } else {
                }
            } else {
                if let Token::Mapping = tokens[n] {
                    mapping.insert(Some(detokenize(&tokens[n + 2])), None);
                    pad = n + 2;
                } else if let Some(_new) = extract_data_types_from_token(&tokens[n]) {
                    mapping.insert(None, Some(MappingValue::Raw(detokenize(&_new))))
                }
            }
        }

        let mut identifier: String = String::new();
        let mut visibility: Option<Token> = None;
        match &tokens[tokens.len() - 2] {
            Token::Identifier(_id) => {
                identifier = _id.to_owned();
            }
            Token::SemiColon => (),
            _ => {
                print_error("Unprocessible entity on mapping");
            }
        }

        if let Token::CloseParenthesis = tokens[tokens.len() - 3] {
            visibility = Some(Token::Internal);
        } else {
            if let Token::Private | Token::Public | Token::External | Token::Internal =
                tokens[tokens.len() - 3]
            {
                if let Token::External = tokens[tokens.len() - 3] {
                    print_error("Mapping can not be set to external");
                } else {
                    visibility = Some(tokens[tokens.len() - 3].to_owned())
                }
            } else {
                print_error("Unprocessible entity on mapping");
            }
        }

        let structured_mapping = MappingIdentifier {
            map: mapping,
            name: identifier,
            visibility: visibility.unwrap(),
        };

        return (None, None, Some(structured_mapping), None);
    } else {
        if let Token::Identifier(_identifier) = &tokens[0] {
            for custom_data_type in custom_data_types {
                if custom_data_type == _identifier {
                    data_type = Some(tokens[0].to_owned());
                    if enums.contains(&_identifier.as_str()) {
                        type_ = VariableType::Enum;
                    } else {
                        if is_function_variable {
                            is_primitive = false;
                        }

                        type_ = VariableType::Struct;
                    }
                }
            }
        } else {
            if let Token::Error = &tokens[0] {
                is_custom_error = true;
            } else if let Token::Event = &tokens[0] {
                is_event = true;
            } else {
                if let Token::String = &tokens[0] {
                    if is_function_variable {
                        is_primitive = false;
                    }
                }
                data_type = Some(tokens[0].to_owned())
            }
        }

        if !is_custom_error && !is_event {
            if let None = data_type {
                print_error(&format!(
                    "Invalid data type \"{}\" on line {}",
                    text.text, text.line
                ));
            }
        }

        if let Token::OpenSquareBracket = &tokens[1] {
            is_array = true;
            if is_function_variable {
                is_primitive = false;
            }
            let close_bracket_index = tokens
                .iter()
                .position(|pred| pred == &Token::CloseSquareBracket);

            if let None = close_bracket_index {
                print_error(&format!("Missing \"]\" on line {}", text.line));
            } else {
                let slice = &tokens[2..close_bracket_index.unwrap()];
                if slice.contains(&Token::Equals) {
                    print_error(&format!("Missing \"]\" on line {}", text.line));
                } else {
                    if close_bracket_index.unwrap() - 1 > 1 {
                        let mut expression = String::new();
                        for slc in slice {
                            let detokenized = LineDescriptions::from_token_to_string(slc);
                            expression.push_str(&detokenized);
                        }
                        size = Some(expression);
                        // size = validate_expression(&expression, text.clone());
                    }
                }
            }
        }

        if !is_primitive {
            if tokens.contains(&Token::Storage) || tokens.contains(&Token::Memory) {
                storage = Some(tokens[1].to_owned())
            } else {
                print_error(
                    &format!("Data location must be \"storage\", \"memory\" or \"calldata\" for variable, but none was given. {}",text.text),
                )
            }
        }

        let equal_token_position = tokens.iter().position(|pred| pred == &Token::Equals);
        if let Some(_position) = equal_token_position {
            let slice_equal_token = &tokens[.._position];

            if let Token::Identifier(_var_name) = &slice_equal_token[slice_equal_token.len() - 1] {
                variable_name = Some(_var_name.to_string());
            } else {
                print_error(&format!("Unprocessible entity {}", text.text))
            }

            let mut _string_value = String::new();

            for res in &tokens[_position + 1..] {
                if let Token::SemiColon = res {
                } else {
                    _string_value.push_str(&LineDescriptions::from_token_to_string(res))
                }
            }

            value = Some(_string_value);
        } else {
            if !is_custom_error && !is_event {
                for token in &tokens {
                    if let Token::Identifier(_val) = token {
                        variable_name = Some(_val.to_owned());
                    }
                }

                if let None = variable_name {
                    print_error(&format!("Unprocessible entity {}", text.text))
                }
            }
        }

        if tokens.contains(&Token::Public) {
            visibility = Token::Public;
        } else if tokens.contains(&Token::Private) {
            visibility = Token::Private;
        } else if tokens.contains(&Token::Internal) {
            visibility = Token::Internal;
        } else if tokens.contains(&Token::External) {
            visibility = Token::External;
        }

        if tokens.contains(&Token::Immutable) {
            mutability = Token::Immutable;
        } else if tokens.contains(&Token::Constant) {
            mutability = Token::Constant;
        }

        if is_custom_error {
            return (None, Some(text.text), None, None);
        }
        if is_event {
            return (None, None, None, Some(text.text));
        }
        let structured = VariableIdentifier {
            data_type: data_type.unwrap(),
            is_array,
            mutability,
            name: variable_name.unwrap(),
            size,
            type_,
            value,
            visibility,
            storage_location: storage,
        };

        return (Some(structured), None, None, None);
    }
}
