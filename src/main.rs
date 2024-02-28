use eval::eval;
use regex::Regex;
use std::{
    env, fs, process,
    time::{self, SystemTime},
};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Identifier(String),
    Contract,
    Require,
    Storage,
    Error,
    Override,
    Push,
    Pop,
    Delete,
    Enum,
    Immutable,
    Mutable,
    Constant,
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

const KEYWORDS: [&str; 38] = [
    "contract",
    "mapping",
    "error",
    "push",
    "pop",
    "immutable",
    "mutable",
    "constant",
    "fallback",
    "calldata",
    "new",
    "cron",
    "delete",
    "receive",
    "gasless",
    "tx",
    "msg",
    "block",
    "constructor",
    "enum",
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
    Enum,
}

#[derive(Debug, Clone)]
struct StructIdentifier {
    identifier: String,
    types: Vec<StructTypes>,
}
#[derive(Debug)]

struct EnumIdentifier {
    identifier: String,
    variants: Vec<String>,
}

#[derive(Debug, Clone)]

struct VariableIdentifier {
    data_type: Token,
    type_: VariableType,
    visibility: Token,
    mutability: Token,
    name: String,
    value: Option<String>,
    is_array: bool,
    size: Option<String>,
}

#[derive(Debug)]
enum OpenedBraceType {
    None,
    Struct,
    Callback,
    Function,
    Contract,
    Enum,
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
            // println!("{}", input);
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
                    combined_char.push_str(character.to_string().as_str());
                } else {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
                // }
            } else {
                combined_char.push_str(character.to_string().as_str());
                if index == input.len() - 1 {
                    lex.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
            }
        }
        for lexed in lex {
            lexems.push(lex_to_token(&lexed));
        }
        // println!("{:?}", lexems)
        lexems
    }
}

fn main() {
    // println!("{:?}", LineDescriptions::to_token("new"));
    // process::exit(1);
    let start_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
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
        .map(|f| f.to_owned().to_string())
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
    let extracted_enums = extract_enum(&structured_stripped_compilable_contents);

    let structs_tree = extract_struct(&structured_stripped_compilable_contents);
    let struct_identifiers: Vec<&str> = structs_tree
        .iter()
        .map(|pred| pred.identifier.as_str())
        .collect();

    let enum_identifiers: Vec<&str> = extracted_enums
        .iter()
        .map(|pred| pred.identifier.as_str())
        .collect();

    let custom_data_types_identifiers: Vec<&str> =
        [enum_identifiers.clone(), struct_identifiers].concat();

    let (global_variables, custom_errors) = extract_global_variables(
        &structured_stripped_compilable_contents,
        &custom_data_types_identifiers,
        &enum_identifiers,
    );
    let functions = extract_functions(
        &structured_stripped_compilable_contents,
        &custom_data_types_identifiers,
        &global_variables,
        &enum_identifiers,
    );
    println!(
        "===> STRUCT ===>\n{:#?}\n\n ===> GLOBAL_VARIABLES ===>\n{:#?}\n\n ===> ENUMS ===>\n{:#?}\n\n ===>> CUSTOM_ERRORS ==>>\n{:#?}\n\n ===>> FUNCTIONS ==>>\n{:#?}",
        structs_tree, global_variables, extracted_enums, custom_errors,functions
    );

    let end_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!(
        "Program completed in \x1b[93m{:?}\x1b[0m",
        (end_time.unwrap() - start_time.unwrap())
    );
}

fn print_error(msg: &str) {
    eprintln!("ERROR: {}", msg);
    process::exit(1);
}

fn custom_data_types_tokens(_type: &Token, data: &Vec<LineDescriptions>) -> Vec<Vec<Token>> {
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
                // if let OpenedBraceType::Struct = opened_brace_type {
                //     combined.push(token)
                // }
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
                // if let OpenedBraceType::Struct = opened_brace_type {
                //     combined.push(token)
                // }
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
                    // opened_brace_type = OpenedBraceType::Enum;
                    // combined.push(token)
                }
            }

            Token::Enum => {
                if let Token::Struct = _type {
                    // opened_brace_type = OpenedBraceType::Struct;
                    // combined.push(token)
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
fn extract_enum(data: &Vec<LineDescriptions>) -> Vec<EnumIdentifier> {
    let extracted_enums = custom_data_types_tokens(&Token::Enum, data);
    let mut enum_identifier: Vec<EnumIdentifier> = Vec::new();

    for enum_inst in extracted_enums {
        let mut _identifier: Option<String> = None;
        if let Token::Identifier(_id) = &enum_inst[1] {
            _identifier = Some(_id.to_string());
        } else {
            print_error("Missing enum identifier!!");
        }

        let stripped = &enum_inst[3..enum_inst.len() - 1];
        let splited: Vec<&[Token]> = stripped.split(|pred| pred == &Token::Coma).collect();
        let mut combined_types: Vec<String> = Vec::new();
        for splited_param in splited.iter().filter(|pred| !pred.is_empty()) {
            if !splited_param.is_empty() {
                if splited_param.len() != 1 {
                    print_error(&format!("Invalid enum variant ",))
                }
                if let Token::Identifier(_id) = &splited_param[0] {
                    validate_identifier_regex(_id, 000);
                    combined_types.push(_id.to_string());
                } else {
                    print_error("Invalid enum variant")
                }
            }
        }

        let structured = EnumIdentifier {
            identifier: _identifier.unwrap(),
            variants: combined_types,
        };

        enum_identifier.push(structured);
    }

    enum_identifier
}

/* *************************** ENUM END ******************************************/

/* *************************** STRUCT START ******************************************/
fn extract_struct(data: &Vec<LineDescriptions>) -> Vec<StructIdentifier> {
    let extracted_structs = custom_data_types_tokens(&Token::Struct, data);
    let mut struct_identifier: Vec<StructIdentifier> = Vec::new();

    for struct_inst in extracted_structs {
        let mut _identifier: Option<String> = None;
        if let Token::Identifier(_id) = &struct_inst[1] {
            _identifier = Some(_id.to_string());
        } else {
            print_error("Missing struct identifier!!");
        }
        let stripped = &struct_inst[3..struct_inst.len() - 1];
        let splited: Vec<&[Token]> = stripped.split(|pred| pred == &Token::SemiColon).collect();
        let mut combined_types: Vec<StructTypes> = Vec::new();
        // println!("{:?}", splited);
        for splited_param in splited.iter().filter(|pred| !pred.is_empty()) {
            let mut type_: Option<String> = None;
            let mut name_: Option<String> = None;
            let mut is_array = false;
            let mut size: Option<String> = None;
            if !splited_param.is_empty() {
                if splited_param.len() < 2 {
                    print_error(&format!("Invalid Struct params ",))
                }

                type_ = Some(format!(
                    "{}",
                    LineDescriptions::from_token_to_string(&splited_param[0],)
                ));

                if let Token::OpenSquareBracket = &splited_param[1] {
                    is_array = true;
                    let close_index = splited_param
                        .iter()
                        .position(|pred| pred == &Token::CloseSquareBracket);

                    if let None = close_index {
                        print_error(&format!(
                            "Syntax error... Expecting a close bracket for struct",
                        ))
                    } else {
                        if close_index.unwrap() - 1 != 1 {
                            if splited_param.len() != 5 {
                                print_error(&format!("Syntax error on struct"))
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

                            match &splited_param[4] {
                                Token::Identifier(_val) => name_ = Some(_val.to_owned()),

                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting identifier but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[4])
                                )),
                            }
                        } else {
                            if splited_param.len() != 4 {
                                print_error(&format!("Syntax error on struct"));
                            }
                            // panic!("sdf");
                            match &splited_param[3] {
                                Token::Identifier(_val) => name_ = Some(_val.to_owned()),

                                _ => print_error(&format!(
                                    "Unprocessible entity.. Expecting identifier but found {}",
                                    LineDescriptions::from_token_to_string(&splited_param[3])
                                )),
                            }
                        }
                    }
                } else if let Token::Identifier(_identifier) = &splited_param[1] {
                    if validate_identifier_regex(&_identifier, 0) {
                        name_ = Some(_identifier.to_owned());
                    }
                } else {
                    print_error(&format!("Invalid struct",))
                }
            }

            if let None = name_ {
                print_error(&format!("Syntax error... missing struct identifier  ",))
            }

            let structured = StructTypes {
                is_array,
                name_: name_.unwrap(),
                size: size,
                type_: type_.unwrap(),
            };
            combined_types.push(structured);
        }

        let struct_build = StructIdentifier {
            identifier: _identifier.unwrap(),
            types: combined_types,
        };
        struct_identifier.push(struct_build);
    }

    struct_identifier
}

/* *************************** STRUCT END ******************************************/

/* *************************** CUSTOM ERROR START ******************************************/
/* *************************** CUSTOM ERROR END ******************************************/

/* *************************** VARIABLE START ******************************************/

fn extract_global_variables(
    data: &Vec<LineDescriptions>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
) -> (Vec<VariableIdentifier>, Vec<String>) {
    let mut global_variables = Vec::new();
    let mut custom_errors: Vec<String> = Vec::new();
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
        let validated = validate_variable(variable, custom_data_types, enums);
        if let Some(_raw) = validated.0 {
            global_variables.push(_raw);
        } else {
            custom_errors.push(validated.1.unwrap())
        }
    }

    (global_variables, custom_errors)
}

fn validate_variable(
    text: LineDescriptions,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
) -> (Option<VariableIdentifier>, Option<String>) {
    let mut is_array = false;
    let mut size: Option<String> = None;
    let mut data_type: Option<Token> = None;
    let mut is_custom_error = false;
    let mut variable_name: Option<String> = None;
    let mut visibility = Token::Private;
    let mut mutability = Token::Mutable;
    let mut value: Option<String> = None;
    let mut type_ = VariableType::Variable;
    let tokens = LineDescriptions::to_token(&format!("{};", text.text));

    if let Token::Identifier(_identifier) = &tokens[0] {
        for custom_data_type in custom_data_types {
            if custom_data_type == _identifier {
                data_type = Some(tokens[0].to_owned());
                if enums.contains(&_identifier.as_str()) {
                    type_ = VariableType::Enum;
                } else {
                    type_ = VariableType::Struct;
                }
            }
        }
    } else {
        if let Token::Error = &tokens[0] {
            is_custom_error = true;
        } else {
            data_type = Some(tokens[0].to_owned())
        }
    }

    if !is_custom_error {
        if let None = data_type {
            print_error(&format!(
                "Invalid data type \"{}\" on line  {}",
                text.text, text.line
            ));
        }
    }

    if let Token::OpenSquareBracket = &tokens[1] {
        is_array = true;

        let close_bracket_index = tokens
            .iter()
            .position(|pred| pred == &Token::CloseSquareBracket);

        if let None = close_bracket_index {
            print_error(&format!("Missing \"]\" on line  {}", text.line));
        } else {
            let slice = &tokens[2..close_bracket_index.unwrap()];
            if slice.contains(&Token::Equals) {
                print_error(&format!("Missing \"]\" on line  {}", text.line));
            } else {
                if close_bracket_index.unwrap() - 1 > 1 {
                    let mut expression = String::new();
                    for slc in slice {
                        let detokenized = LineDescriptions::from_token_to_string(slc);
                        expression.push_str(&detokenized);
                    }
                    size = validate_expression(&expression, text.clone());
                }
            }
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
        if !is_custom_error {
            for token in &tokens {
                if let Token::Identifier(_val) = token {
                    variable_name = Some(_val.to_owned());
                }
            }

            if let None = variable_name {
                print_error(&format!("Unprocessibledd entity {}", text.text))
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
        return (None, Some(text.text));
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
    };

    (Some(structured), None)
}

/* *************************** EXPRESSION VALIDATION START ******************************************/

fn validate_expression(expression: &String, text: LineDescriptions) -> Option<String> {
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

/* *************************** EXPRESSION VALIDATION END ******************************************/

/* *************************** EXPRESSION EVALUATION START ******************************************/

fn evaluate_expression(expression: &String, text: LineDescriptions) -> Option<String> {
    let mut expr = String::new();
    if expression.ends_with(";") {
        expr = expression[..expression.len() - 1].to_string();
    } else {
        expr = expression.to_owned();
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
                            if fractional_part < 1.0 {
                                print_error(&format!(
                                    "Uprocessible entity for array size \"0\". {}... ",
                                    text.text
                                ));
                            } else {
                                size = Some(_dec.trunc().to_string());
                            }
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

/* *************************** EXPRESSION EVALUATION END ******************************************/

/* *************************** FUNCTION START ******************************************/

fn extract_functions(
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
        );

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
            let mut is_primitive = false;
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

            if is_primitive {
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

            if is_primitive {
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
    )
}

fn extract_function_block(
    arms: &Vec<Vec<&Token>>,
    custom_data_types: &Vec<&str>,
    enums: &Vec<&str>,
    global_variables: &Vec<VariableIdentifier>,
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
            Token::If => {
                println!("if condition");
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
                // let open_brace_index = block.iter().position(|pred| pred == &&Token::OpenBraces);

                // if let Some(_position) = open_brace_index {
                //     let vars: Vec<Token> = block[_position + 1..block.len() - 1]
                //         .to_vec()
                //         .iter()
                //         .map(|pred| pred.to_owned().to_owned())
                //         .collect();

                //   let res =   extract_function_arms(&vars, custom_data_types, global_variables, enums);
                //   println!("RESPONSE HERE =>>> {:?}", res)
                // }

                // println!("For");
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
                } else {
                    print_error(&format!("Unexpected identifier \"{}\"", detokenize(_token)))
                }
            }
        }
    }
    // println!("{:#?}========>> \n\n\n\n\n", full_block);
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
    let variable = validate_variable(LineDescriptions { text, line: 0 }, custom_data_types, enums);
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
                    variant: Some(detokenize(block[2])),
                    type_: VariableAssignType::Enum,
                    value,
                }))
            } else if let VariableType::Struct = _var.type_ {
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

#[derive(Debug)]
struct FunctionIdentifier {
    name: String,
    gasless: bool,
    visibility: Token,
    arguments: Vec<Argument>,
    returns: Option<Vec<ReturnType>>,
    r#override: bool,
    r#virtual: bool,
    arms: Vec<FunctionArm>,
}

#[derive(Debug)]
enum VariableAssignType {
    Expression,
    Struct,
    Enum,
    Array(Option<String>),
}
#[derive(Debug)]
struct VariableAssign {
    identifier: String,
    value: String,
    variant: Option<String>,
    operation: VariableAssignOperation,
    type_: VariableAssignType,
}
#[derive(Debug)]
struct Delete {
    identifier: String,
    type_: VariableAssignType,
    variant: Option<String>,
    data_type: Token,
}

#[derive(Debug)]
enum VariableAssignOperation {
    Push,
    Pop,
    Assign,
}

#[derive(Debug)]
struct Return {
    value: String,
}

#[derive(Debug)]
struct FunctionCall {
    identifier: String,
    arguments: Vec<String>,
}
#[derive(Debug)]
struct Require {
    condition: String,
    message: Option<String>,
}

#[derive(Debug)]
struct ElIf {
    condition: Vec<Token>,
    arm: Vec<FunctionArm>,
}
#[derive(Debug)]
struct Conditionals {
    condition: Vec<Token>,
    arm: Vec<FunctionArm>,
    elif: Option<Vec<ElIf>>,
    el: Option<Vec<FunctionArm>>,
}

#[derive(Debug)]
enum FunctionArm {
    VariableIdentifier(VariableIdentifier),
    VariableAssign(VariableAssign),
    FunctionCall(FunctionCall),
    Require(Require),
    Conditionals(Conditionals),
    Return(Return),
    Delete(Delete),
}

struct For {
    start: String,
    condition: String,
    increment: String,
    arms: Vec<FunctionArm>,
}

enum FunctionArmType {
    StructAssign,
    VariableAssign,
    Conditional,
    Require,
    For,
    None,
}

fn lex_to_token(input: &str) -> Token {
    let token = match input {
        "contract" => Token::Contract,
        "mapping" => Token::Mapping,
        "storage" => Token::Storage,
        "delete" => Token::Delete,
        "push" => Token::Push,
        "pop" => Token::Pop,
        "msg" => Token::Msg,
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

fn detokenize(input: &Token) -> String {
    let token: String = match input {
        Token::Contract => "contract".to_string(),
        Token::Storage => "storage".to_string(),
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

fn extract_data_location_from_token(token: &Token) -> Option<Token> {
    match token {
        Token::Memory => Some(Token::Memory),
        Token::Calldata => Some(Token::Calldata),
        _ => None,
    }
}

fn extract_integer_types_from_token(token: &Token) -> Option<Token> {
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
