use std::{env, fs, process};

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

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Identifier(String),
    Contract,
    Mapping,
    Calldata,
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
    Upgradable,
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
    Immutable,
    Constant,
    Enum,
    Mutable,
}

#[derive(Debug)]
enum TreeExpression {
    VariableIdentifier(Token, Token, Token, String, Option<Vec<Token>>, bool),
    //*DATATYPE, VISIBILITY, NAME, VALUE, ARRAY*/;
    StructIdentifier(String, Vec<Argument>),
    //* STRUCT NAME, DATA STRUCTURE */
    FunctionIdentifier(FunctionIdentifier),
    CallbackIdentifier(CallbackIdentifier),
}

#[derive(Debug, Clone)]
enum Expression {
    Contract,
    Callback,
    Struct,
    Function,
    Mapping,
    None,
}

#[derive(Debug)]

enum Argument {
    Params(Token, String, bool),
    //DATATYPE, NAME, isarray
}

fn main() {
    /// DEFINE KEYWORDS
    const KEYWORDS: [&str; 62] = [
        "contract",
        "mapping",
        "fallback",
        "calldata",
        "cron",
        "receive",
        "gasless",
        "tx",
        "msg",
        "block",
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
        "upgrdable",
        "constant",
        "immutable",
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

    /* GET ENVIRONMENT ARGUMENTS */
    let args: Vec<String> = env::args().collect();

    /* CHECK FOR VALID ARGUMENTS */
    if args.len() < 2 {
        print_error("Mising file path... Run cargo run <file-path>")
    }

    /* VALIDATE FILE FORMAT */
    if args[1].split(".").last().unwrap() != "sol" {
        print_error("Unsupported file... Expected \".sol\" file");
    }

    /* READ FILE TO STRING */
    let file_contents = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        print_error(&format!("Could not read file. {err}"));
        process::exit(1);
    });

    /* STRIP COMMENTS AND WHITE SPACES FROM FILE STRINGS */
    let stripped_comments: Vec<&str> = file_contents
        .lines()
        .filter(|pred| !pred.trim().starts_with("//") && !pred.trim().is_empty())
        .collect();

    /* STRIPED INLINE COMMENTS */
    let mut stripped_inline_comments: Vec<String> = Vec::new();

    /* STRIP INLINE COMMENTS */
    for stripped_comment in stripped_comments.iter() {
        let index = stripped_comment.find("//");
        if let Some(index_value) = index {
            stripped_inline_comments.push(stripped_comment[..index_value].trim().to_string())
        } else {
            stripped_inline_comments.push(stripped_comment.trim().to_string())
        }
    }

    /* JOIN STRIPPED INLINE COMMENTS */
    let mut joined_stripped_string = stripped_inline_comments.join("");

    /* STRIP DOC STRINGS */
    while joined_stripped_string.contains("/*") || joined_stripped_string.contains("*/") {
        let str_start_index = joined_stripped_string.find("/*");
        let str_end_index = joined_stripped_string.find("*/");

        if let Some(index_) = str_start_index {
            if let Some(_index) = str_end_index {
                let left: String = joined_stripped_string[..index_].to_string();
                let right: String = joined_stripped_string[_index + 2..].to_string();

                joined_stripped_string = left + &right;
            }
        }
    }

    /* COMBINED RELATED CHARACTERS */
    let mut combined_char = String::new();
    /* LEX STRING */
    let mut lex_string: Vec<String> = Vec::new();
    /* CHARACTERS REGEX IDENTIFIER */
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();

    /* GROUP RELATED CHARACTERS LIKE B O O L TO BOOL */
    for character in joined_stripped_string.chars() {
        if character.is_whitespace() && !combined_char.trim().is_empty() {
            lex_string.push(combined_char.trim().to_string());
            combined_char.clear();
        } else if let Some(_) = KEYWORDS
            .iter()
            .find(|pred| pred == &&character.to_string().as_str())
        {
            if !combined_char.trim().is_empty() {
                lex_string.push(combined_char.trim().to_string());
                combined_char.clear();
            }
            lex_string.push(character.to_string());
        } else if let Some(_) = KEYWORDS
            .iter()
            .find(|pred| pred == &&combined_char.as_str().trim())
        {
            if let Some(_) = identifier_regex.find(character.to_string().as_str()) {
                combined_char.push_str(character.to_string().as_str())
            } else {
                lex_string.push(combined_char.trim().to_string());
                combined_char.clear();
            }
        } else {
            combined_char.push_str(character.to_string().as_str())
        }
    }

    let mut lexems: Vec<Token> = Vec::new();
    let mut parsed_expression: Vec<TreeExpression> = Vec::new();
    for lexed in lex_string {
        lexems.push(lex_string_to_token(&lexed));
    }

    let parsed_tokens: Vec<Vec<Token>> = parse_tokens_into_collections(lexems);
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

    println!("{:#?}", parsed_expression)
}

fn print_error(msg: &str) {
    println!("Error: {}", msg);
    process::exit(1);
}

/* PARSE LEX STRING TO TOKENS */
fn lex_string_to_token(input: &str) -> Token {
    let token = match input {
        "contract" => Token::Contract,
        "mapping" => Token::Mapping,
        "upgradable" => Token::Upgradable,
        "immutable" => Token::Immutable,
        "constant" => Token::Constant,
        "enum" => Token::Enum,
        "calldata" => Token::Calldata,
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
        "mutable" => Token::Mutable,
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

fn detokenize(input: Token) -> String {
    let token = match input {
        Token::Contract => "contract".to_string(),
        Token::Mapping => "mapping".to_string(),
        Token::Mutable => "mutable".to_string(),
        Token::Upgradable => "upgradable".to_string(),
        Token::Immutable => "immutable".to_string(),
        Token::Constant => "constant".to_string(),
        Token::Enum => "enum".to_string(),
        Token::Calldata => "calldata".to_string(),
        Token::Msg => "msg".to_string(),
        Token::Constructor => "constructor".to_string(),
        Token::Receive => "receive".to_string(),
        Token::Fallback => "fallback".to_string(),
        Token::Cron => "cron".to_string(),
        Token::Gasless => "gasless".to_string(),
        Token::Address => "address".to_string(),
        Token::Private => "private".to_string(),
        Token::Struct => "struct".to_string(),
        Token::Function => "function".to_string(),
        Token::Public => "public".to_string(),
        Token::View => "view".to_string(),
        Token::Returns => "returns".to_string(),
        Token::Pure => "pure".to_string(),
        Token::Return => "return".to_string(),
        Token::External => "external".to_string(),
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

        Token::Identifier(val_) => val_.to_string(),
    };
    token
}

/* PARSE TOKENS TO RELATIVE COLLECTIONS */
fn parse_tokens_into_collections(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut current_expression = Expression::None;
    let mut expression: Vec<Token> = Vec::new();
    let mut expression_parent: Vec<Vec<Token>> = Vec::new();
    let mut opened_braces = 0;

    for token in tokens {
        match token {
            Token::Contract => {
                current_expression = Expression::Contract;
                expression.push(token.clone());
            }
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
                if opened_braces == 0 {
                    expression_parent.push(expression.clone());
                    expression.clear();
                } else {
                    if let Expression::Contract = current_expression {
                    } else {
                        expression.push(token);
                    }
                }
                opened_braces += 1;
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

                if opened_braces - 1 == 1 {
                    current_expression = Expression::Contract;
                } else if opened_braces - 1 == 0 {
                    current_expression = Expression::None
                }

                opened_braces -= 1
            }

            _ => expression.push(token),
        }
    }
    expression_parent
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

fn extract_Mutability_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Constant => Some(Token::Constant),
        Token::Immutable => Some(Token::Immutable),

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

fn parse_variable(tokens: Vec<Token>) -> TreeExpression {
    // if tokens.len() > 8 {
    //     panic!("ERROR: {tokens:?}")
    // }
    if tokens.len() < 3 {
        panic!("ERROR: {tokens:?}")
    }
    let data_type = &tokens[0];
    let mut visibility: Option<Token> = None;
    let mut mutability: Option<Token> = Some(Token::Mutable);
    let mut name: Option<String> = None;
    let mut value: Option<Vec<Token>> = None;

    let start_index = tokens.iter().position(|pred| pred == &Token::Equals);
    // println!("{:?}", &tokens[start_index.unwrap() + 1..tokens.len() - 1]);
    if let Some(_) = start_index {
        value = Some(tokens[start_index.unwrap() + 1..tokens.len() - 1].to_vec());
    }
    for id in &tokens[1..] {
        if let Some(_) = extract_visibility_from_token(&id) {
            visibility = extract_visibility_from_token(&id);
        } else {
            if let Some(_) = extract_Mutability_from_token(&id) {
                mutability = extract_Mutability_from_token(&id);
            }
            match id {
                Token::Identifier(_predicate) => {
                    if let None = name {
                        name = Some(_predicate.clone())
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

    let final_expression = TreeExpression::VariableIdentifier(
        data_type.clone(),
        visibility.unwrap(),
        mutability.unwrap(),
        name.unwrap(),
        value.clone(),
        is_array,
    );

    final_expression
}

fn parse_structs(tokens: Vec<Token>) -> TreeExpression {
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
    let expr = TreeExpression::StructIdentifier(struct_name.unwrap().clone(), args);
    expr
}

fn parse_function(tokens: Vec<Token>) -> TreeExpression {
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

    TreeExpression::FunctionIdentifier(structured)
}

fn parse_callback(tokens: Vec<Token>) -> TreeExpression {
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

    TreeExpression::CallbackIdentifier(structured)
}
