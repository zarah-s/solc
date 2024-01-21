use std::{env, fmt::Error, fs, process};

const KEYWORDS: [&str; 52] = [
    "contract",
    "mapping",
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
    Views,
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
            lex.push(combined_char.trim().to_string());
            combined_char.clear();
        } else {
            combined_char.push_str(character.to_string().as_str())
        }
    }

    for lexed in lex {
        lexems.push(lex_to_token(&lexed));
    }

    let parsed_tokens: Vec<Vec<Token>> = parse_token(lexems);

    for parse in parsed_tokens {
        let init = &parse[0];

        if let Some(_d) = extract_data_types_from_token(&init) {
            parsed_expression.push(parse_variable(parse))
        } else {
            // println!("{init:?}")
        }
        // match init {
        //     Token::Identifier(_d) => {
        //         // println!("{_d:?} {parse:?}")
        //     }
        //     _ => (),
        // }
    }

    println!("{:?}", parsed_expression)
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
        "address" => Token::Address,
        "private" => Token::Private,
        "struct" => Token::Struct,
        "function" => Token::Function,
        "public" => Token::Public,
        "views" => Token::Views,
        "returns" => Token::Returns,
        "pure" => Token::Pure,
        "return" => Token::Return,
        "external" => Token::External,
        "payable" => Token::Payable,
        "memory" => Token::Memory,
        "uint" => Token::Uint,
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
enum ParsedExpression {
    VariableIdentifier(Token, Token, String, Option<String>),
    //DATATYPE, VISIBILITY, NAME, VALUE;
}

fn extract_callback_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Constructor => Some(Token::Constructor),
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
                _ => (),
            }
        }
    }

    if let None = visibility {
        visibility = Some(Token::Private);
    }

    let final_expression = ParsedExpression::VariableIdentifier(
        data_type.clone(),
        visibility.unwrap(),
        name.unwrap(),
        value.clone(),
    );

    final_expression
}
