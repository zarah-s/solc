use std::{env, fs, process};

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Identifier(String),
    Contract,
    Override,
    Internal,
    External,
    Virtual,
    Calldata,
    New,
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

const DATA_TYPES: [&str; 28] = [
    "uint8",
    "uint8[]",
    "uint16",
    "uint16[]",
    "uint",
    "uint[]",
    "uint32",
    "uint32[]",
    "bytes1",
    "bytes1[]",
    "uint256",
    "uint256[]",
    "int",
    "int[]",
    "int8",
    "int8[]",
    "int16",
    "int16[]",
    "int32",
    "int32[]",
    "int256",
    "int256[]",
    "bool",
    "bool[]",
    "string",
    "string[]",
    "address",
    "address[]",
];

const KEYWORDS: [&str; 29] = [
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
    "internal",
    "memory",
    "if",
    "else",
    "for",
    "upgrdable",
    "constant",
    "immutable",
];

const SYMBOLS: [&str; 22] = [
    "+", "-", "/", "*", "(", ")", "[", "]", "{", "}", ">", "<", ".", "=", "!", "%", ";", "\"", "'",
    ",", "|", "&",
];

#[derive(Debug, Clone)]

struct StructTypes {
    type_: String,
    name_: String,
    size: Option<String>,
    is_array: bool,
}

// enum Argument<'a> {
//     Arg(&'a str, &'a str),
// }

#[derive(Debug)]
struct Argument {
    type_: String,
    name_: String,
    location: Option<Token>,
    size: Option<String>,
    is_array: bool,
}

#[derive(Debug)]
struct ReturnType {
    type_: String,
    location: Option<Token>,
    size: Option<String>,
    is_array: bool,
}

#[derive(Debug, Clone)]

enum VariableType {
    Variable,
    Struct,
    Contract,
}

#[derive(Debug, Clone)]
struct StructIdentifier {
    identifier: String,
    types: Vec<StructTypes>,
}

#[derive(Debug, Clone)]

struct VariableIdentifier {
    data_type: String,
    type_: VariableType,
    visibility: String,
    mutability: String,
    name: String,
    value: Option<String>,
}

impl VariableIdentifier {
    pub fn new(
        data_type: String,
        visibility: String,
        type_: VariableType,
        mutability: String,
        name: String,
        value: Option<String>,
    ) -> Self {
        Self {
            data_type,
            visibility,
            type_,
            mutability,
            name,
            value,
        }
    }
}

#[derive(Debug)]
enum OpenedBraceType {
    None,
    Struct,
    Callback,
    Function,
    Contract,
}

impl StructIdentifier {
    pub fn new(identifier: String, types: Vec<StructTypes>) -> Self {
        Self { identifier, types }
    }
}

#[derive(Debug, Clone)]
struct LineDescriptions {
    text: String,
    line: i32,
}

impl LineDescriptions {
    fn to_string(self) -> String {
        format!("{}&=>{}%%", self.text, self.line)
    }

    fn to_struct(value: String) -> Vec<Self> {
        let splited: Vec<&str> = value.split("%%").collect();
        let mut return_value: Vec<Self> = Vec::new();
        for split in splited {
            let val: Vec<&str> = split.split("&=>").collect();
            if !val.first().unwrap().trim().is_empty() {
                return_value.push(Self {
                    text: val.first().unwrap().to_string(),
                    line: val.last().unwrap().parse().unwrap(),
                })
            }
        }

        return_value
    }

    fn from_token_to_string(token: &Token) -> String {
        return detokenize(&token);
    }

    fn to_token(input: &str) -> Vec<Token> {
        let mut lex: Vec<String> = Vec::new();
        let mut combined_char = String::new();
        let mut lexems: Vec<Token> = Vec::new();
        let mut opened_quotations = 0;
        let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
        for (index, character) in input.chars().enumerate() {
            if character == '"' || character == '\'' {
                if opened_quotations == 0 {
                    opened_quotations += 1;
                } else {
                    opened_quotations = 0
                }
            }
            if character.is_whitespace() && !combined_char.trim().is_empty() {
                if opened_quotations > 0 {
                    combined_char.push_str(character.to_string().as_str())
                } else {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
            } else if let Some(_) = SYMBOLS
                .iter()
                .find(|pred| pred == &&character.to_string().as_str())
            {
                if !combined_char.trim().is_empty() {
                    if opened_quotations > 0 {
                        combined_char.push_str(character.to_string().as_str())
                    } else {
                        lex.push(combined_char.trim().to_string());
                        combined_char.clear();
                    }
                }
                lex.push(character.to_string());
            } else if let Some(_) = [KEYWORDS.to_vec(), DATA_TYPES.to_vec(), SYMBOLS.to_vec()]
                .concat()
                .iter()
                .find(|pred| pred == &&combined_char.as_str().trim())
            {
                // if let Some(_next) = input.chars().nth(index + 1) {
                //     let mut joined = String::from(combined_char.clone());
                //     joined.push(character);
                //     joined.push(_next);

                //     if [DATA_TYPES.to_vec()]
                //         .concat()
                //         .concat()
                //         .contains(&joined.as_str().trim())
                //     {
                //         combined_char.push_str(character.to_string().as_str())
                //     } else {
                //         panic!("here {}", joined);
                //         lex.push(combined_char.trim().to_string());
                //         combined_char.clear();
                //     }
                // } else {
                if let Some(_) = identifier_regex.find(character.to_string().as_str()) {
                    combined_char.push_str(character.to_string().as_str())
                } else {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
                // }
            } else {
                combined_char.push_str(character.to_string().as_str())
            }
        }
        for lexed in lex {
            lexems.push(lex_to_token(&lexed));
        }
        lexems
    }
}

fn main() {
    /* GET ENVIRONMENT ARGUMENTS */
    let args: Vec<String> = env::args().collect();

    /* LINES DESCRIPTION CONTAINING LINE NUMBER */
    let mut lines_: Vec<LineDescriptions> = Vec::new();

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

    for (index, content) in file_contents.lines().enumerate() {
        lines_.push(LineDescriptions {
            line: (index as i32) + 1,
            text: content.to_string(),
        })
    }

    /* STRIP COMMENTS AND WHITE SPACES FROM LINE DESCRIPTORS */
    let stripped_comments: Vec<&LineDescriptions> = lines_
        .iter()
        .filter(|pred| !pred.text.trim().starts_with("//") && !pred.text.trim().is_empty())
        .collect();

    /* STRIPED INLINE COMMENTS */
    let mut stripped_inline_comments: Vec<LineDescriptions> = Vec::new();

    /* STRIP INLINE COMMENTS */
    for stripped_comment in stripped_comments.iter() {
        let comment_index = stripped_comment.text.find("//");
        let doc_str_index = stripped_comment.text.find("/*");
        if let Some(index_value) = comment_index {
            stripped_inline_comments.push(LineDescriptions {
                text: stripped_comment.text[..index_value].trim().to_string(),
                ..**stripped_comment
            })
        } else {
            if let Some(index_value) = doc_str_index {
                stripped_inline_comments.push(LineDescriptions {
                    text: stripped_comment.text[..index_value].trim().to_string(),
                    ..**stripped_comment
                })
            } else {
                stripped_inline_comments.push(LineDescriptions {
                    text: stripped_comment.text.trim().to_string(),
                    ..**stripped_comment
                })
            }
        }
    }

    /* JOIN STRIPPED INLINE COMMENTS */
    let joined_stripped_vec: Vec<String> = stripped_inline_comments
        .iter()
        .map(|f| f.clone().to_string())
        .collect();

    let mut joined_stripped_string = String::new();
    for sst in joined_stripped_vec {
        joined_stripped_string.push_str(sst.as_str());
    }

    /* STRIP DOC STRINGS */
    while joined_stripped_string.contains(&"/*".to_string())
        || joined_stripped_string.contains(&"*/".to_string())
    {
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

    let structured_stripped_compilable_contents: Vec<LineDescriptions> =
        LineDescriptions::to_struct(joined_stripped_string);

    extract_custom_data_types(&structured_stripped_compilable_contents);
    // let custom_data_types_identifiers: Vec<&str> = structs_tree
    //     .iter()
    //     .map(|pred| pred.identifier.as_str())
    //     .collect();

    // let global_variables = extract_global_variables(
    //     &structured_stripped_compilable_contents,
    //     &custom_data_types_identifiers,
    // );
    // extract_functions(
    //     &structured_stripped_compilable_contents,
    //     &custom_data_types_identifiers,
    // );
    // println!("{:#?} {:#?}", structs_tree, global_variables)
}

fn print_error(msg: &str) {
    eprintln!("ERROR: {}", msg);
    process::exit(1);
}

fn validate_identifier_regex(identifer: &str, line: i32) -> bool {
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

/* *************************** ENUM START ******************************************/

/* *************************** STRUCT START ******************************************/
fn extract_custom_data_types(data: &Vec<LineDescriptions>) {
    let mut stringified = String::new();
    let mut extracted_structs: Vec<Vec<Token>> = Vec::new();
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
                if let OpenedBraceType::Struct = opened_brace_type {
                    combined.push(token)
                }
            }

            Token::CloseBraces => {
                if let OpenedBraceType::Struct = opened_brace_type {
                    combined.push(token)
                }
                opened_braces -= 1;
                if opened_braces == 1 {
                    opened_brace_type = OpenedBraceType::None;
                    if !combined.is_empty() {
                        extracted_structs.push(combined.clone());
                        combined.clear();
                    }
                }
            }
            Token::Struct => {
                opened_brace_type = OpenedBraceType::Struct;
                combined.push(token)
            }

            _other => {
                if let OpenedBraceType::Struct = opened_brace_type {
                    combined.push(_other)
                }
            }
        }
    }

    for struct_inst in extracted_structs {
        println!("{:#?}", struct_inst);
    }
}

// fn validate_struct(data: &Vec<LineDescriptions>) -> StructIdentifier {
//     let mut identifier: Option<&str> = None;
//     let mut types: Vec<StructTypes> = Vec::new();
//     for sst in data {
//         if sst.text.starts_with("struct") {
//             let splited_str: Vec<&str> = sst.text.split(" ").collect();
//             if splited_str.len() < 2 {
//                 print_error(&format!(
//                     "Unprocessible entity \"{}\" on line {}",
//                     sst.text, sst.line
//                 ))
//             } else {
//                 if validate_identifier_regex(splited_str[1], sst.line) {
//                     identifier = Some(splited_str[1]);
//                 }
//             }

//             let check_inline_format: Vec<&str> = sst.text.split("{").collect();
//             if check_inline_format.len() < 2 {
//                 print_error(&format!(
//                     "Unprocessible entity {} on line {}",
//                     sst.text, sst.line
//                 ))
//             } else {
//                 if check_inline_format.len() > 0 && !check_inline_format[1].is_empty() {
//                     let inline_types: Vec<&str> = check_inline_format[1].split(";").collect();
//                     for inline in inline_types {
//                         if inline != "}" && !inline.is_empty() {
//                             if let Some(return_value) =
//                                 validate_struct_type(&format!("{inline};"), sst.line)
//                             {
//                                 types.push(return_value);
//                             }
//                         }
//                     }
//                 }
//             }
//         } else {
//             if sst.text != "}" {
//                 if let Some(return_value) = validate_struct_type(&sst.text, sst.line) {
//                     types.push(return_value);
//                 }
//             }
//         }
//     }

//     StructIdentifier::new(identifier.unwrap().to_string(), types)
// }

// fn validate_struct_type(text: &str, line: i32) -> Option<StructTypes> {
//     let splited: Vec<&str> = text.split(" ").collect();
//     if splited.len() != 2 {
//         print_error(&format!(
//             "Unprocessible entity \"{}\" on line {}",
//             text, line
//         ));
//         None
//     } else {
//         if !text.ends_with(";") {
//             print_error(&format!("Expecting \"{}\" on line {}", ";", line));
//             None
//         } else {
//             if !DATA_TYPES.contains(&splited[0]) {
//                 print_error(&format!(
//                     "Unidentified identifier \"{}\" on line {}",
//                     splited[0], line
//                 ));
//                 None
//             } else {
//                 let splited_terminate: Vec<&str> = splited[1].split(";").collect();
//                 if validate_identifier_regex(splited_terminate[0], line) {
//                     return Some(StructTypes::Type(
//                         splited[0].to_string(),
//                         splited_terminate[0].to_string(),
//                     ));
//                 }
//                 None
//             }
//         }
//     }
// }

/* *************************** STRUCT END ******************************************/

/* *************************** VARIABLE START ******************************************/

fn extract_global_variables(
    data: &Vec<LineDescriptions>,
    custom_data_types: &Vec<&str>,
) -> Vec<VariableIdentifier> {
    let mut global_variables = Vec::new();
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
                    && !sst.text.contains("mapping")
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
        global_variables.push(validate_variable(variable, custom_data_types));
    }

    global_variables
}

fn validate_variable(text: LineDescriptions, custom_data_types: &Vec<&str>) -> VariableIdentifier {
    let spl: Vec<&str> = text.text.split(" ").collect();

    let data_type = spl[0];
    let mut visibility = "private";
    let mut mutability = "mutable";
    let mut value: Option<String> = None;
    let splited: Vec<&str> = text.text.split("=").collect();
    let left_padding: Vec<&str> = splited[0].split(" ").collect();
    let name = left_padding
        .iter()
        .filter(|pred| !pred.is_empty())
        .last()
        .unwrap();
    if splited.len() > 1 {
        value = Some(splited[1].trim().to_string())
    }

    if let Some(_visibility) = Regex::new(r"\b(public|private|external)\b")
        .unwrap()
        .find(splited[0])
    {
        visibility = _visibility.as_str();
    }

    if let Some(_mutability) = Regex::new(r"\b(constant|immutable)\b")
        .unwrap()
        .find(splited[0])
    {
        mutability = _mutability.as_str();
    }
    let mut validated_type = false;
    let mut variable_type: Option<VariableType> = None;
    for sst in [DATA_TYPES.to_vec(), custom_data_types.to_vec()].concat() {
        if data_type.ends_with("]") {
            if sst.starts_with(&data_type[..data_type.len() - 2]) {
                validated_type = true;
            }
        } else {
            if sst.starts_with(data_type) {
                validated_type = true;
            }
        }
    }

    if !validated_type {
        print_error(&format!(
            "Invalid data type \"{}\" on line  {}",
            data_type, text.line
        ));
    } else {
        if let Some(_) = DATA_TYPES.iter().find(|pred| pred == &&data_type) {
            variable_type = Some(VariableType::Variable);
        } else {
            variable_type = Some(VariableType::Struct);
        }
    }

    if let None = variable_type {
        print_error(&format!(
            "Invalid data type \"{}\" on line  {}",
            data_type, text.line
        ));
    }

    let structured = VariableIdentifier::new(
        data_type.to_string(),
        visibility.to_string(),
        variable_type.unwrap(),
        mutability.to_string(),
        name.to_string(),
        value,
    );
    structured
    // println!("{:#?}", structured)
}

/* *************************** VARIABLE START ******************************************/

/* *************************** FUNCTION START ******************************************/

fn extract_functions(data: &Vec<LineDescriptions>, custom_data_types: &Vec<&str>) {
    let mut opened_braces = 0;
    let mut opened_braces_type = OpenedBraceType::None;
    let mut processed_data: Vec<Vec<LineDescriptions>> = Vec::new();
    let mut combined = Vec::new();
    for line in data {
        let raw = line.text.clone();

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
        // single_structure.push(processed);
        for prr in processed {
            combined.push_str(&prr.text);
        }

        stringified.push(combined.clone());
        combined.clear();
    }

    for single_stringified in stringified {
        let tokens = LineDescriptions::to_token(single_stringified.as_str());
        // println!("{:#?}", tokens);
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
        let mut function_visibility = Token::Internal;
        let mut function_returns: Option<Vec<ReturnType>> = None;
        let start_params = function_definition
            .iter()
            .position(|pred| pred == &Token::OpenParenthesis);
        let end_params = function_definition
            .iter()
            .position(|pred| pred == &Token::CloseParenthesis);

        let params_block = &function_definition[start_params.unwrap_or_else(|| {
            print_error("Shitty syntax");
            0
        }) + 1..end_params.unwrap_or_else(|| {
            print_error("Shitty syntax");
            0
        })];

        let splited_params_block: Vec<&[Token]> =
            params_block.split(|pred| pred == &Token::Coma).collect();
        let function_arguments = extract_function_params(splited_params_block, function_definition);
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

        if function_definition.contains(&Token::Override) {
            function_override = true;
        }

        if function_definition.contains(&Token::Virtual) {
            function_virtual = true;
        }

        println!(
            "{:?} \n {:#?}\n {:#?}\n {function_override}\n {function_virtual}\n {:#?} \n\n\n\n",
            function_name, function_visibility, function_arguments, function_returns
        );
    }
}

fn extract_function_params(
    splited_params_block: Vec<&[Token]>,
    function_definition: &[Token],
) -> Vec<Argument> {
    let mut function_arguments: Vec<Argument> = Vec::new();

    for splited_param in splited_params_block {
        if !splited_param.is_empty() {
            let mut type_: Option<String> = None;
            let mut name_: Option<String> = None;
            let mut location_: Option<Token> = None;
            let mut is_array = false;
            let mut size: Option<String> = None;
            let mut is_primitive = false;
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
                    if let Token::String = splited_param[0] {
                        is_primitive = true;
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
                    is_primitive = true;
                    let close_index = splited_param
                        .iter()
                        .position(|pred| pred == &Token::CloseSquareBracket);

                    if let None = close_index {
                        print_error(&format!(
                            "Syntax error... Expecting a close bracket for {}",
                            vec_.join(" ")
                        ))
                    } else {
                        if close_index.unwrap() - 1 != 1 {
                            if splited_param.len() != 6 {
                                print_error(&format!("Syntax error {}... Missing \"memory\" or \"calldata\" or argument identifier", vec_.join(" "),))
                            }
                            if let None = extract_data_location_from_token(&splited_param[4]) {
                                print_error(&format!(
                                    "Expecting \"memory\" or \"calldata\". {} ",
                                    vec_.join(" "),
                                ))
                            } else {
                                location_ = extract_data_location_from_token(&splited_param[4]);
                            }
                            match &splited_param[2] {
                                Token::Identifier(_val) => {
                                    if let Some(_dd) = Regex::new(r"^\d+$").unwrap().find(_val) {
                                        if _val == "0" {
                                            print_error(&format!("Invalid array size {}", _val))
                                        }
                                        size = Some(_val.to_owned());
                                    } else {
                                        print_error(&format!("Invalid array size {}", _val))
                                    }
                                }
                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting a size of uint but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[2])
                                )),
                            }

                            match &splited_param[5] {
                                Token::Identifier(_val) => name_ = Some(_val.to_owned()),

                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting identifier but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[5])
                                )),
                            }
                        } else {
                            if splited_param.len() != 5 {
                                print_error(&format!("Syntax error {}... Missing \"memory\" or \"calldata\" or argument identifier", vec_.join(" "),))
                            }
                            if let None = extract_data_location_from_token(&splited_param[3]) {
                                print_error(&format!(
                                    "Expecting \"memory\" or \"calldata\". {} ",
                                    vec_.join(" "),
                                ))
                            } else {
                                location_ = extract_data_location_from_token(&splited_param[3]);
                            }

                            match &splited_param[4] {
                                Token::Identifier(_val) => name_ = Some(_val.to_owned()),

                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting identifier but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[4])
                                )),
                            }
                        }
                    }
                } else if let Some(_location) = extract_data_location_from_token(&splited_param[1])
                {
                    if !is_primitive {
                        print_error(&format!(
                            "Syntax error... cannot declare \"{}\" on a  non-primive type. {} ",
                            LineDescriptions::from_token_to_string(&_location),
                            vec_.join(" "),
                        ))
                    } else {
                        location_ = Some(_location);
                        if let Token::Identifier(_identifier) = &splited_param[2] {
                            if validate_identifier_regex(&_identifier, 0) {
                                name_ = Some(_identifier.to_owned());
                            }
                        } else {
                            print_error(&format!("Invalid function argument {}", vec_.join(" ")))
                        }
                    }
                } else if let Token::Identifier(_identifier) = &splited_param[1] {
                    if validate_identifier_regex(&_identifier, 0) {
                        name_ = Some(_identifier.to_owned());
                    }
                } else {
                    print_error(&format!("Invalid function argument {}", vec_.join(" ")))
                }
            }

            if let None = name_ {
                print_error(&format!(
                    "Syntax error... missing argument identifier {:?} ",
                    vec_.join(" "),
                ))
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
            let mut is_primitive = false;

            let vec_: Vec<String> = function_definition
                .iter()
                .map(|pred| LineDescriptions::from_token_to_string(pred))
                .collect();

            if DATA_TYPES
                .contains(&LineDescriptions::from_token_to_string(&splited_param[0]).as_str())
            {
                if let Token::String = splited_param[0] {
                    is_primitive = true;
                }
                type_ = Some(format!(
                    "{}",
                    LineDescriptions::from_token_to_string(&splited_param[0],)
                ));
            } else {
                if custom_data_types
                    .contains(&LineDescriptions::from_token_to_string(&splited_param[0]).as_str())
                {
                    is_primitive = true;
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
                    is_primitive = true;
                    let close_index = splited_param
                        .iter()
                        .position(|pred| pred == &Token::CloseSquareBracket);

                    if let None = close_index {
                        print_error(&format!(
                            "Syntax error... Expecting a close bracket for {}",
                            vec_.join(" ")
                        ))
                    } else {
                        if close_index.unwrap() - 1 != 1 {
                            if splited_param.len() != 5 {
                                print_error(&format!("Syntax error {}... Missing \"memory\" or \"calldata\" or argument identifier", vec_.join(" "),))
                            }
                            if let None = extract_data_location_from_token(&splited_param[4]) {
                                print_error(&format!(
                                    "Expecting \"memory\" or \"calldata\". {} ",
                                    vec_.join(" "),
                                ))
                            } else {
                                location_ = extract_data_location_from_token(&splited_param[4]);
                            }
                            match &splited_param[2] {
                                Token::Identifier(_val) => {
                                    if let Some(_dd) = Regex::new(r"^\d+$").unwrap().find(_val) {
                                        if _val == "0" {
                                            print_error(&format!("Invalid array size {}", _val))
                                        }
                                        size = Some(_val.to_owned());
                                    } else {
                                        print_error(&format!("Invalid array size {}", _val))
                                    }
                                }
                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting a size of uint but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[2])
                                )),
                            }
                        } else {
                            if splited_param.len() != 4 {
                                print_error(&format!("Syntax error {}... Missing \"memory\" or \"calldata\" or argument identifier", vec_.join(" "),))
                            }
                            if let None = extract_data_location_from_token(&splited_param[3]) {
                                print_error(&format!(
                                    "Expecting \"memory\" or \"calldata\". {} ",
                                    vec_.join(" "),
                                ))
                            } else {
                                location_ = extract_data_location_from_token(&splited_param[3]);
                            }
                        }
                    }
                } else if let Some(_location) = extract_data_location_from_token(&splited_param[1])
                {
                    if !is_primitive {
                        print_error(&format!(
                            "Syntax error... cannot declare \"{}\" on a  non-primive type. {} ",
                            LineDescriptions::from_token_to_string(&_location),
                            vec_.join(" "),
                        ))
                    } else {
                        location_ = Some(_location);
                    }
                } else {
                    print_error(&format!("Invalid function argument {}", vec_.join(" ")))
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

#[derive(Debug, Clone)]
enum FunctionIdentifier {
    Other(Vec<String>),
    // Variable(VariableIdentifier),
    Condition(Conditionals),
}

#[derive(Debug, Clone)]

struct ElIf {
    condition: Vec<Token>,
    arm: Vec<FunctionIdentifier>,
}

#[derive(Debug, Clone)]

struct Conditionals {
    condition: Vec<Token>,
    arm: Vec<FunctionIdentifier>,
    elif: Option<Vec<ElIf>>,
    el: Option<Vec<FunctionIdentifier>>,
}

impl Conditionals {
    pub fn new(input: &str) {
        println!("{input}")
    }
}

fn lex_to_token(input: &str) -> Token {
    let token = match input {
        "contract" => Token::Contract,
        "mapping" => Token::Mapping,
        "msg" => Token::Msg,
        "constructor" => Token::Constructor,
        "receive" => Token::Receive,
        "internal" => Token::Internal,
        "external" => Token::External,
        "calldata" => Token::Calldata,
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
        "override" => Token::Override,
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

fn detokenize(input: &Token) -> String {
    let token: String = match input {
        Token::Contract => "contract".to_string(),
        Token::Mapping => "mapping".to_string(),
        Token::Msg => "msg".to_string(),
        Token::Constructor => "constructor".to_string(),
        Token::Calldata => "calldata".to_string(),
        Token::Receive => "receive".to_string(),
        Token::Fallback => "fallback".to_string(),
        Token::Cron => "cron".to_string(),
        Token::Virtual => "virtual".to_string(),
        Token::New => "new".to_string(),
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
        Token::Return => "return".to_string(),
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

fn extract_data_location_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Memory => Some(Token::Memory),
        Token::Calldata => Some(Token::Calldata),
        _ => None,
    }
}
