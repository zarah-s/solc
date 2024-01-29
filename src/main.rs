use std::{env, fs, io::Write};

use regex::Regex;

#[derive(Debug)]
struct FunctionIdentifier {
    name: String,
    arguments: Option<Vec<Argument>>,
    visibility: Token,
    view: Option<Token>,
    return_type: Option<String>,
    gasless: bool,
    payable: bool,
    arms: Vec<Vec<Token>>,
}

#[derive(Debug)]

struct CallbackIdentifier {
    type_: Token,
    arguments: Option<Vec<Argument>>,
    arms: Vec<Vec<Token>>,
}

impl FunctionIdentifier {
    pub fn new(
        name: String,
        visibility: Token,
        view: Option<Token>,
        arms: Vec<Vec<Token>>,
        return_type: Option<String>,
        gasless: bool,
        payable: bool,
        arguments: Option<Vec<Argument>>,
    ) -> Self {
        Self {
            name,
            visibility,
            view,
            return_type,
            gasless,
            arms,
            payable,
            arguments,
        }
    }
}

impl CallbackIdentifier {
    pub fn new(type_: Token, arguments: Option<Vec<Argument>>, arms: Vec<Vec<Token>>) -> Self {
        Self {
            type_,
            arguments,
            arms,
        }
    }
}

const KEYWORDS: [&str; 56] = [
    "contract",
    "mapping",
    "fallback",
    "cron",
    "receive",
    "gasless",
    "msg",
    "constructor",
    "address",
    "private",
    "struct",
    "function",
    "public",
    "views",
    "returns",
    "pure",
    "return",
    "external",
    "memory",
    "uint8",
    "uint16",
    "uint32",
    "uint120",
    "uint256",
    "int8",
    "int16",
    "int32",
    "int120",
    "int256",
    "string",
    "bool",
    "if",
    "else",
    "for",
    "+",
    "-",
    "/",
    "*",
    "(",
    ")",
    "[",
    "]",
    "{",
    "}",
    ">",
    "<",
    ".",
    "=",
    "!",
    "%",
    ";",
    "\"",
    "'",
    ",",
    "|",
    "&",
];

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Identifier(String),
    Contract,
    Mapping,
    Msg,
    Constructor,
    Address,
    Private,
    Struct,
    Function,
    Public,
    View,
    Returns,
    Pure,
    Return,
    External,
    Memory,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint120,
    Uint256,
    Receive,
    Fallback,
    Payable,
    Cron,
    Gasless,
    Int8,
    Int,
    Int16,
    Int32,
    Int120,
    Int256,
    String,
    Bool,
    If,
    Else,
    For,
    Plus,
    Minus,
    Divide,
    Multiply,
    OpenParenthesis,
    CloseParenthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenBraces,
    CloseBraces,
    GreaterThan,
    LessThan,
    Dot,
    Equals,
    Bang,
    Modulu,
    SemiColon,
    Quotation,
    Coma,
    Pipe,
    Ampersand,
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("File required")
    }

    let file_contents = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        panic!("ERROR: Error reading file. {}, \"{}\"", err, args[1]);
    });

    let stripped: String = file_contents
        .lines()
        .filter(|pred| !pred.trim().starts_with("//"))
        .collect();
    let mut lex: Vec<String> = Vec::new();
    let mut combined_char = String::new();
    let mut lexems: Vec<Token> = Vec::new();
    let mut parsed_expression: Vec<ParsedExpression> = Vec::new();
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();

    for character in stripped.chars() {
        if character.is_whitespace() && !combined_char.trim().is_empty() {
            lex.push(combined_char.trim().to_string());
            combined_char.clear();
        } else if let Some(_) = KEYWORDS
            .iter()
            .find(|pred| pred == &&character.to_string().as_str())
        {
            if !combined_char.trim().is_empty() {
                lex.push(combined_char.trim().to_string());
                combined_char.clear();
            }
            lex.push(character.to_string());
        } else if let Some(_) = KEYWORDS
            .iter()
            .find(|pred| pred == &&combined_char.as_str().trim())
        {
            // println!("{combined_char} {}", characters[index]);

            if let Some(_) = identifier_regex.find(character.to_string().as_str()) {
                combined_char.push_str(character.to_string().as_str())
            } else {
                lex.push(combined_char.trim().to_string());
                combined_char.clear();
            }
        } else {
            combined_char.push_str(character.to_string().as_str())
        }
    }

    for lexed in lex {
        lexems.push(lex_to_token(&lexed));
    }

    let parsed_tokens: Vec<Vec<Token>> = parse_token(lexems);
    // println!("{:?}", parsed_tokens);

    for parse in parsed_tokens {
        let init = &parse[0];

        if let Some(_) = extract_data_types_from_token(&init) {
            parsed_expression.push(parse_variable(parse))
        } else {
            match init {
                Token::Struct => parsed_expression.push(parse_structs(parse)),
                Token::Function => parsed_expression.push(parse_function(parse)),

                _other => {
                    if let Some(_) = extract_callback_from_token(&init) {
                        parsed_expression.push(parse_callback(parse))
                    }
                }
            }
        }
    }

    // println!("{:#?}", parsed_expression)
}

fn lex_to_token(input: &str) -> Token {
    let token = match input {
        "contract" => Token::Contract,
        "mapping" => Token::Mapping,
        "msg" => Token::Msg,
        "constructor" => Token::Constructor,
        "receive" => Token::Receive,
        "fallback" => Token::Fallback,
        "cron" => Token::Cron,
        "gasless" => Token::Gasless,
        "address" => Token::Address,
        "private" => Token::Private,
        "struct" => Token::Struct,
        "function" => Token::Function,
        "public" => Token::Public,
        "view" => Token::View,
        "returns" => Token::Returns,
        "pure" => Token::Pure,
        "return" => Token::Return,
        "external" => Token::External,
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

fn detokenize(input: Token) -> &'static str {
    let token = match input {
        Token::Contract => "contract",
        Token::Mapping => "mapping",
        Token::Msg => "msg",
        Token::Constructor => "constructor",
        Token::Receive => "receive",
        Token::Fallback => "fallback",
        Token::Cron => "cron",
        Token::Gasless => "gasless",
        Token::Address => "address",
        Token::Private => "private",
        Token::Struct => "struct",
        Token::Function => "function",
        Token::Public => "public",
        Token::View => "view",
        Token::Returns => "returns",
        Token::Pure => "pure",
        Token::Return => "return",
        Token::External => "external",
        Token::Payable => "payable",
        Token::Memory => "memory",
        Token::Uint => "uint",
        Token::Int => "int",
        Token::Int8 => "int8",
        Token::Uint8 => "uint8",
        Token::Uint16 => "uint16",
        Token::Uint32 => "uint32",
        Token::Uint120 => "uint120",
        Token::Uint256 => "uint256",
        Token::Int16 => "int16",
        Token::Int32 => "int32",
        Token::Int120 => "int120",
        Token::Int256 => "int256",
        Token::String => "string",
        Token::Bool => "bool",
        Token::If => "if",
        Token::Else => "else",
        Token::For => "for",
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Divide => "/",
        Token::Multiply => "*",
        Token::OpenParenthesis => "(",
        Token::CloseParenthesis => ")",
        Token::OpenSquareBracket => "[",
        Token::CloseSquareBracket => "]",
        Token::OpenBraces => "{",
        Token::CloseBraces => "}",
        Token::GreaterThan => ">",
        Token::LessThan => "<",
        Token::Dot => ".",
        Token::Equals => "=",
        Token::Bang => "!",
        Token::Modulu => "%",
        Token::SemiColon => ";",
        Token::Quotation => "\"",
        Token::Coma => ",",
        Token::Pipe => "|",
        Token::Ampersand => "&",

        _ => "",
    };
    token
}

fn parse_token(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut current_expression = Expression::None;
    let mut expression: Vec<Token> = Vec::new();
    let mut expression_parent: Vec<Vec<Token>> = Vec::new();
    let mut opened_braces = 0;

    for token in tokens {
        if let Expression::Contract = current_expression {}
        match token {
            Token::Contract => current_expression = Expression::Contract,
            Token::Constructor => {
                current_expression = Expression::Callback;
                expression.push(token.clone());
            }
            Token::Receive => {
                current_expression = Expression::Callback;
                expression.push(token.clone());
            }
            Token::Cron => {
                current_expression = Expression::Callback;
                expression.push(token.clone());
            }
            Token::Fallback => {
                current_expression = Expression::Callback;
                expression.push(token.clone());
            }
            Token::Struct => {
                current_expression = Expression::Struct;
                expression.push(token);
            }
            Token::Mapping => {
                current_expression = Expression::Mapping;
                expression.push(token);
            }
            Token::Function => {
                current_expression = Expression::Function;
                expression.push(token);
            }
            Token::SemiColon => {
                expression.push(token.clone());

                if let Expression::Contract = current_expression {
                    current_expression = Expression::Contract;
                    if !expression.is_empty() {
                        expression_parent.push(expression.clone());
                        expression.clear();
                    }
                }

                if let Expression::Mapping = current_expression {
                    current_expression = Expression::Contract;
                    expression_parent.push(expression.clone());
                    expression.clear();
                }
            }
            Token::OpenBraces => {
                opened_braces += 1;
                if let Expression::Contract = current_expression {
                } else {
                    expression.push(token);
                }
            }
            Token::CloseBraces => {
                if let Expression::Function = current_expression {
                    if opened_braces - 1 == 1 {
                        expression.push(token.clone());
                        expression_parent.push(expression.clone());

                        expression.clear();
                        current_expression = Expression::Contract;
                    } else {
                        expression.push(token.clone());
                    }
                } else {
                    expression.push(token);
                    expression_parent.push(expression.clone());
                    expression.clear();
                    current_expression = Expression::Contract;
                }

                // if let Expression::Struct = current_expression {
                // }

                if opened_braces - 1 == 1 {
                    current_expression = Expression::Contract;
                } else if opened_braces - 1 == 0 {
                    current_expression = Expression::None
                }

                opened_braces -= 1
            }

            _ => {
                // if let Some(_) = extract_data_types_from_token(&token) {
                if opened_braces > 0 {
                    expression.push(token)
                }
                // }
            }
        }
    }
    // println!("{expression_parent:?}");
    expression_parent
}

#[derive(Debug, Clone)]
enum Expression {
    Contract,
    Callback,
    Struct,
    Function,
    Variable,
    Mapping,
    None,
}

#[derive(Debug)]

enum Argument {
    Params(Token, String, bool),
    //DATATYPE, NAME, isarray
}
#[derive(Debug)]
enum ParsedExpression {
    VariableIdentifier(Token, Token, String, Option<String>, bool),
    //*DATATYPE, VISIBILITY, NAME, VALUE, ARRAY*/;
    StructIdentifier(String, Vec<Argument>),
    //* STRUCT NAME, DATA STRUCTURE */
    FunctionIdentifier(FunctionIdentifier),
    CallbackIdentifier(CallbackIdentifier),
}

fn extract_callback_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Constructor => Some(Token::Constructor),
        Token::Fallback => Some(Token::Fallback),
        Token::Receive => Some(Token::Receive),
        Token::Cron => Some(Token::Cron),
        _ => None,
    }
}

fn extract_data_types_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Uint => Some(Token::Uint),
        Token::Uint8 => Some(Token::Uint8),
        Token::Uint16 => Some(Token::Uint16),
        Token::Uint32 => Some(Token::Uint32),
        Token::Uint120 => Some(Token::Uint120),
        Token::Uint256 => Some(Token::Uint256),
        Token::Int => Some(Token::Int),
        Token::Int8 => Some(Token::Int8),
        Token::Int16 => Some(Token::Int16),
        Token::Int32 => Some(Token::Int32),
        Token::Int120 => Some(Token::Int120),
        Token::Int256 => Some(Token::Int256),
        Token::String => Some(Token::String),
        Token::Bool => Some(Token::Bool),
        Token::Address => Some(Token::Address),
        _ => None,
    }
}

fn extract_visibility_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Private => Some(Token::Private),
        Token::Public => Some(Token::Public),
        Token::External => Some(Token::External),
        _ => None,
    }
}

fn parse_variable(tokens: Vec<Token>) -> ParsedExpression {
    println!("{tokens:?}");
    if tokens.len() > 8 {
        panic!("ERROR: {tokens:?}")
    }
    if tokens.len() < 3 {
        panic!("ERROR: {tokens:?}")
    }
    let data_type = &tokens[0];
    let mut visibility: Option<Token> = None;
    let mut name: Option<String> = None;
    let mut value: Option<String> = None;

    for id in &tokens[1..] {
        if let Some(_) = extract_visibility_from_token(&id) {
            visibility = extract_visibility_from_token(&id);
        } else {
            match id {
                Token::Identifier(_predicate) => {
                    if let None = name {
                        name = Some(_predicate.clone())
                    } else {
                        value = Some(_predicate.clone());
                    }
                }
                // Token::OpenSquareBracket
                _ => (),
            }
        }
    }

    if let None = visibility {
        visibility = Some(Token::Private);
    }

    let is_array = if tokens.contains(&Token::OpenSquareBracket) {
        true
    } else {
        false
    };

    let final_expression = ParsedExpression::VariableIdentifier(
        data_type.clone(),
        visibility.unwrap(),
        name.unwrap(),
        value.clone(),
        is_array,
    );

    final_expression
}

fn parse_structs(tokens: Vec<Token>) -> ParsedExpression {
    let struct_name: &Token = &tokens[1];
    let mut args: Vec<Argument> = Vec::new();

    let start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
    let dd: &Vec<&[Token]> = &tokens[start_index.unwrap() + 1..tokens.len() - 1]
        .split(|pred| pred == &Token::SemiColon)
        .collect();

    for fg in dd {
        if !fg.is_empty() {
            if fg.contains(&Token::OpenSquareBracket) {
                let name = match &fg[3] {
                    Token::Identifier(_name) => Some(_name),
                    _ => None,
                };
                if let None = name {
                    panic!("ERROR");
                }

                let arg = Argument::Params(fg[0].clone(), name.unwrap().clone(), true);
                args.push(arg);
            } else {
                let name = match &fg[1] {
                    Token::Identifier(_name) => Some(_name),
                    _ => None,
                };
                if let None = name {
                    panic!("ERROR here {fg:?}");
                }

                let arg = Argument::Params(fg[0].clone(), name.unwrap().clone(), false);
                args.push(arg);
            }
        }
    }
    let struct_name = match struct_name {
        Token::Identifier(_name) => Some(_name),
        _ => None,
    };

    if let None = struct_name {
        panic!("ERROR: INVALID STRUCT NAME");
    }
    let expr = ParsedExpression::StructIdentifier(struct_name.unwrap().clone(), args);
    expr
}

fn parse_function(tokens: Vec<Token>) -> ParsedExpression {
    let function_name = match &tokens[1] {
        Token::Identifier(_name) => Some(_name),
        _ => None,
    };

    let mut visibility: Option<Token> = None;
    let mut is_gasless = false;
    let mut payable: bool = false;
    let mut view: Option<Token> = None;
    let mut arms: Vec<Vec<Token>> = Vec::new();
    let mut args: Vec<Argument> = Vec::new();

    let args_start_index = tokens
        .iter()
        .position(|pred| pred == &Token::OpenParenthesis);
    let args_end_index = tokens
        .iter()
        .position(|pred| pred == &Token::CloseParenthesis);

    if let Some(_index) = args_start_index {
        let slice = &tokens[_index + 1..args_end_index.unwrap()];
        let joined: Vec<&[Token]> = slice.split(|pred| pred == &Token::Coma).collect();

        for arr in joined {
            if !arr.is_empty() {
                // println!("{arr:?}");
                if arr.len() < 2 {
                    panic!("ERROR: Invalid argument")
                } else {
                    let identifier = match &arr[arr.len() - 1] {
                        Token::Identifier(_val) => Some(_val),
                        _ => None,
                    };

                    let is_array = arr.contains(&Token::OpenSquareBracket);
                    if let None = identifier {
                        panic!("ERROR: Identifier not found")
                    }
                    args.push(Argument::Params(
                        arr[0].clone(),
                        identifier.unwrap().clone(),
                        is_array,
                    ))
                }
            }
        }
    }

    let arms_start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
    if let Some(_index) = arms_start_index {
        let slice = &tokens[_index + 1..tokens.len() - 1];
        let joined: Vec<&[Token]> = slice.split(|pred| pred == &Token::SemiColon).collect();
        for arm in joined {
            if !arm.is_empty() {
                arms.push(arm.to_vec());
            }
        }
    }

    if tokens.contains(&Token::View) {
        view = Some(Token::View)
    } else if tokens.contains(&Token::Pure) {
        view = Some(Token::Pure)
    }

    for fnd in &tokens[1..] {
        if let Some(_) = extract_visibility_from_token(fnd) {
            visibility = Some(fnd.clone());
        } else {
            match fnd {
                Token::Gasless => is_gasless = true,
                Token::Payable => payable = true,
                _ => (),
            }
        }
    }

    if let None = function_name {
        panic!("ERROR: INVALID FUNCTION NAME");
    }

    let args = if args.is_empty() { None } else { Some(args) };

    let structured = FunctionIdentifier::new(
        function_name.unwrap().clone(),
        visibility.unwrap(),
        view,
        arms,
        None,
        is_gasless,
        payable,
        args,
    );

    ParsedExpression::FunctionIdentifier(structured)
}

fn parse_callback(tokens: Vec<Token>) -> ParsedExpression {
    let type_ = &tokens[0];

    let mut arms: Vec<Vec<Token>> = Vec::new();
    let mut args: Vec<Argument> = Vec::new();

    let args_start_index = tokens
        .iter()
        .position(|pred| pred == &Token::OpenParenthesis);
    let args_end_index = tokens
        .iter()
        .position(|pred| pred == &Token::CloseParenthesis);

    if let Some(_index) = args_start_index {
        let slice = &tokens[_index + 1..args_end_index.unwrap()];
        let joined: Vec<&[Token]> = slice.split(|pred| pred == &Token::Coma).collect();
        for arr in joined {
            if !arr.is_empty() {
                // println!("{arr:?}");
                if arr.len() < 2 {
                    panic!("ERROR: Invalid argument")
                } else {
                    let identifier = match &arr[arr.len() - 1] {
                        Token::Identifier(_val) => Some(_val),
                        _ => None,
                    };

                    let is_array = arr.contains(&Token::OpenSquareBracket);
                    if let None = identifier {
                        panic!("ERROR: Identifier not found")
                    }
                    args.push(Argument::Params(
                        arr[0].clone(),
                        identifier.unwrap().clone(),
                        is_array,
                    ))
                }
            }
        }
    }

    let arms_start_index = tokens.iter().position(|pred| pred == &Token::OpenBraces);
    if let Some(_index) = arms_start_index {
        let slice = &tokens[_index + 1..tokens.len() - 1];
        let joined: Vec<&[Token]> = slice.split(|pred| pred == &Token::SemiColon).collect();
        for arm in joined {
            if !arm.is_empty() {
                arms.push(arm.to_vec());
            }
        }
    }

    let args = if args.is_empty() { None } else { Some(args) };

    let structured = CallbackIdentifier::new(type_.clone(), args, arms);

    ParsedExpression::CallbackIdentifier(structured)
}
