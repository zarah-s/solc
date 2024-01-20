use std::{borrow::Borrow, env, fmt::format, fs, process};

use regex::Regex;

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
    // "uint",
    "uint8",
    "uint16",
    "uint32",
    "uint120",
    "uint256",
    // "int",
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

#[derive(Debug, Clone)]
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
    Int,
    Int8,
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

#[derive(Debug)]
#[allow(dead_code)]
enum Scope {
    Global,
    Functional(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Argument {
    Params(String, String),
    //DATATYPE, NAME
}

#[derive(Debug)]
#[allow(dead_code)]

struct StructIdentifier {
    identifier: String,
    types: Vec<Argument>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct FunctionIdentifier {
    name: String,
    arguments: Option<Vec<Argument>>,
    visibility: String,
    view: Option<String>,
    return_type: Option<String>,
    gasless: bool,
    payable: bool,

    arms: Vec<Token>,
}

#[derive(Clone, Copy)]

enum BraceType {
    Function,
    Struct,
    None,
}

enum OpenedBraces {
    Value(BraceType, i8),
}

impl FunctionIdentifier {
    pub fn new(
        name: String,
        visibility: String,
        view: Option<String>,
        arms: Vec<Token>,
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

#[derive(Debug)]

enum Expression {
    VariableIdentifier(String, String, String, Option<String>, Scope),
    //DATATYPE, VISIBILITY, NAME, VALUE;
    FunctionIdentifier(FunctionIdentifier),
    Require(String, String),
    Struct(StructIdentifier),
    Assignment(String, String),
    StructVariableIdentifier(String, String, Vec<Argument>),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expecting a file path but got none");
        process::exit(1);
    }

    let mut tokens: Vec<Token> = Vec::new();

    let file_content = fs::read_to_string(args[1].trim())
        .expect(&format!("Could not open file \"<{}>\"", args[1]));

    let striped_contents: Vec<&str> = file_content
        .lines()
        .filter(|pred| !pred.trim().starts_with("//") && !pred.is_empty())
        .collect();

    let mut lex: Vec<String> = Vec::new();

    for line in &striped_contents {
        let mut combined_char = String::new();
        for (_, chard) in line.trim().chars().enumerate() {
            let character = chard.to_string().to_string();

            if chard.is_whitespace() && !combined_char.trim().is_empty() {
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
                combined_char.push_str(&character)
            }
        }
    }

    for lexed in lex {
        tokens.push(lex_to_token(&lexed));
    }

    parse(tokens);
    // println!("{tokens:?}");
}

fn lex_to_token(input: &str) -> Token {
    let token = match input {
        "contract" => Token::Contract,
        "mapping" => Token::Mapping,
        "msg" => Token::Msg,
        "constructor" => Token::Constructor,
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
        "memory" => Token::Memory,
        "uint" => Token::Uint,
        "uint8" => Token::Uint8,
        "uint16" => Token::Uint16,
        "uint32" => Token::Uint32,
        "uint120" => Token::Uint120,
        "uint256" => Token::Uint256,
        "int" => Token::Int,
        "int8" => Token::Int8,
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

#[derive(Debug)]
enum ExpressionType {
    Variable,
    Struct,
    Function,
    Mapping,
    Callback,
    Identifier,
}

fn parse(tokens: Vec<Token>) {
    let mut opened_braces: OpenedBraces = OpenedBraces::Value(BraceType::None, 0);
    let mut current_expr_type: Option<ExpressionType> = None;
    let mut expr_parent: Vec<Vec<Token>> = Vec::new();
    let mut expr: Vec<Token> = vec![];
    for token in tokens {
        let data_type = extract_data_types_from_token(&token);
        let callback_type = extract_callback_from_token(&token);
        let visibility = extract_visibility_from_token(&token);

        let has_opened_braces = match opened_braces {
            OpenedBraces::Value(opened, _) => match opened {
                BraceType::None => false,
                _ => true,
            },
        };

        if let Some(_) = data_type {
            if !has_opened_braces {
                current_expr_type = Some(ExpressionType::Variable);
            }
        } else if let Some(_) = callback_type {
            current_expr_type = Some(ExpressionType::Callback);
        } else if let Some(_) = visibility {
        } else {
            match token {
                Token::Function => {
                    current_expr_type = Some(ExpressionType::Function);
                }
                Token::Struct => {
                    current_expr_type = Some(ExpressionType::Struct);
                }

                Token::Mapping => {
                    current_expr_type = Some(ExpressionType::Mapping);
                }
                Token::OpenBraces => {
                    let opened_braces_count = match opened_braces {
                        OpenedBraces::Value(_, count) => count,
                    };
                    if let Some(val) = &current_expr_type {
                        match val {
                            ExpressionType::Struct => {
                                opened_braces =
                                    OpenedBraces::Value(BraceType::Struct, opened_braces_count + 1);
                            }
                            _ => (),
                        }
                    }
                    // current_expr_type = Some(ExpressionType::Mapping);
                }
                Token::CloseBraces => {
                    let opened_braces_count = match opened_braces {
                        OpenedBraces::Value(_, count) => count,
                    };
                    if let Some(val) = &current_expr_type {
                        match val {
                            ExpressionType::Struct => {
                                opened_braces =
                                    OpenedBraces::Value(BraceType::Struct, opened_braces_count - 1);
                            }
                            _ => (),
                        }
                    }
                    // current_expr_type = Some(ExpressionType::Mapping);
                }

                Token::Contract => {
                    // current_expr_type = Some(ExpressionType::Mapping);
                }

                Token::Identifier(_) => {
                    // current_expr_type = Some(ExpressionType::Identifier);
                }
                _ => {
                    // println!("signs {token:?}");
                }
            }
        }

        if let Some(cr) = &current_expr_type {
            match cr {
                ExpressionType::Variable => {
                    if let Token::SemiColon = token {
                        expr.push(token);
                        expr_parent.push(expr.clone());
                        expr.clear();
                    } else {
                        expr.push(token);
                    }
                }

                ExpressionType::Struct => {
                    if let Token::CloseBraces = token {
                        expr.push(token);
                        expr_parent.push(expr.clone());
                        expr.clear();
                    } else {
                        expr.push(token);
                    }
                }

                // ExpressionType::Struct => {}
                _ => {
                    // println!("Empty {cr:?}")
                }
            }
        }

        // if let Some(dd) = &current_expr_type {
        //     match dd {
        //         ExpressionType::Variable => {

        //         }
        //         _ => (),
        //     }
        // }

        // println!("{current_expr_type:?} {token:?}");
    }

    println!("{expr_parent:?}")
}

fn extract_callback_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Constructor => Some(Token::Constructor),
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
