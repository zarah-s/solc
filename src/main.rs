use std::{env, fs, process, string};

use regex::Regex;

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

const KEYWORDS: [&str; 28] = [
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

enum StructTypes {
    Type(String, String),
}

#[derive(Debug, Clone)]
struct StructIdentifier {
    identifier: String,
    types: Vec<StructTypes>,
}

#[derive(Debug, Clone)]

struct VariableIdentifier {
    data_type: String,
    visibility: String,
    mutability: String,
    name: String,
    value: Option<String>,
}

impl VariableIdentifier {
    pub fn new(
        data_type: String,
        visibility: String,
        mutability: String,
        name: String,
        value: Option<String>,
    ) -> Self {
        Self {
            data_type,
            visibility,
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
        let index = stripped_comment.text.find("//");
        if let Some(index_value) = index {
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

    let structs_tree = extract_custom_data_types(&structured_stripped_compilable_contents);
    let global_variables = extract_global_variables(&structured_stripped_compilable_contents);
    extract_functions(&structured_stripped_compilable_contents);
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
            "Expecting struct identifier \"{}\" on line {}",
            identifer, line
        ));
        false
    } else {
        if let Some(_) = Regex::new(r"[\W]").unwrap().find(identifer) {
            print_error(&format!(
                "Invalid struct Identifier \"{}\" on line {}",
                identifer, line
            ));
            false
        } else {
            if let Some(_id) = identifier_regex.find(identifer) {
                true
            } else {
                print_error(&format!(
                    "Invalid struct Identifier \"{}\" on line {}",
                    identifer, line
                ));
                false
            }
        }
    }
}

/* *************************** STRUCT START ******************************************/
fn extract_custom_data_types(data: &Vec<LineDescriptions>) -> Vec<StructIdentifier> {
    let stringified_vec_data: Vec<String> = data.iter().map(|ff| ff.clone().to_string()).collect();
    let mut stringified_data = stringified_vec_data.join("");
    let mut stringified_structs: Vec<String> = Vec::new();
    let mut structs: Vec<Vec<LineDescriptions>> = Vec::new();
    let mut struct_trees: Vec<StructIdentifier> = Vec::new();

    while stringified_data.contains("struct") {
        let start_index = stringified_data.find("struct");
        if let Some(_) = start_index {
            let left_padding = stringified_data[start_index.unwrap()..].to_string();
            let end_index = &left_padding.find("}");
            if let None = end_index {
                print_error(&format!(
                    "Unprocessible entity on line {:?}",
                    LineDescriptions::to_struct(left_padding.clone())
                        .last()
                        .unwrap()
                        .line
                ))
            }
            let position_from_close_brace = &left_padding[end_index.unwrap()..].find("%%");

            let right_padding = left_padding
                [..end_index.unwrap() + (position_from_close_brace.unwrap() + 2)]
                .to_string();
            stringified_structs.push(right_padding);
            stringified_data = stringified_data[..start_index.unwrap()].to_string()
                + &stringified_data[start_index.unwrap() + end_index.unwrap()..].to_string()
        }
    }

    for stt in stringified_structs {
        structs.push(LineDescriptions::to_struct(stt))
    }

    for sst in structs {
        if !sst.first().unwrap().text.contains("structor") {
            struct_trees.push(validate_struct(&sst))
        }
    }
    struct_trees
}

fn validate_struct(data: &Vec<LineDescriptions>) -> StructIdentifier {
    let mut identifier: Option<&str> = None;
    let mut types: Vec<StructTypes> = Vec::new();
    for sst in data {
        if sst.text.starts_with("struct") {
            let splited_str: Vec<&str> = sst.text.split(" ").collect();
            if splited_str.len() < 2 {
                print_error(&format!(
                    "Unprocessible entity \"{}\" on line {}",
                    sst.text, sst.line
                ))
            } else {
                if validate_identifier_regex(splited_str[1], sst.line) {
                    identifier = Some(splited_str[1]);
                }
            }

            let check_inline_format: Vec<&str> = sst.text.split("{").collect();
            if check_inline_format.len() < 2 {
                print_error(&format!(
                    "Unprocessible entity {} on line {}",
                    sst.text, sst.line
                ))
            } else {
                if check_inline_format.len() > 0 && !check_inline_format[1].is_empty() {
                    let inline_types: Vec<&str> = check_inline_format[1].split(";").collect();
                    for inline in inline_types {
                        if inline != "}" && !inline.is_empty() {
                            if let Some(return_value) =
                                validate_struct_type(&format!("{inline};"), sst.line)
                            {
                                types.push(return_value);
                            }
                        }
                    }
                }
            }
        } else {
            if sst.text != "}" {
                if let Some(return_value) = validate_struct_type(&sst.text, sst.line) {
                    types.push(return_value);
                }
            }
        }
    }

    StructIdentifier::new(identifier.unwrap().to_string(), types)
}

fn validate_struct_type(text: &str, line: i32) -> Option<StructTypes> {
    let splited: Vec<&str> = text.split(" ").collect();
    if splited.len() != 2 {
        print_error(&format!(
            "Unprocessible entity \"{}\" on line {}",
            text, line
        ));
        None
    } else {
        if !text.ends_with(";") {
            print_error(&format!("Expecting \"{}\" on line {}", ";", line));
            None
        } else {
            // if !DATA_TYPES.contains(&splited[0]) {
            //     print_error(&format!(
            //         "Unidentified identifier \"{}\" on line {}",
            //         splited[0], line
            //     ));
            //     None
            // } else {
            let splited_terminate: Vec<&str> = splited[1].split(";").collect();
            if validate_identifier_regex(splited_terminate[0], line) {
                return Some(StructTypes::Type(
                    splited[0].to_string(),
                    splited_terminate[0].to_string(),
                ));
            }
            None
            // }
        }
    }
}

/* *************************** STRUCT END ******************************************/

/* *************************** VARIABLE START ******************************************/

fn extract_global_variables(data: &Vec<LineDescriptions>) -> Vec<VariableIdentifier> {
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
        global_variables.push(validate_variable(variable));
    }

    global_variables
}

fn validate_variable(text: LineDescriptions) -> VariableIdentifier {
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

    let structured = VariableIdentifier::new(
        data_type.to_string(),
        visibility.to_string(),
        mutability.to_string(),
        name.to_string(),
        value,
    );
    structured
    // println!("{:#?}", structured)
}

/* *************************** VARIABLE START ******************************************/

/* *************************** FUNCTION START ******************************************/

fn extract_functions(data: &Vec<LineDescriptions>) {
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
                }
            }
        }

        if raw.starts_with("function") {
            opened_braces_type = OpenedBraceType::Function;
        }

        if let OpenedBraceType::Function = opened_braces_type {
            if opened_braces == 1 {
                opened_braces_type = OpenedBraceType::Contract;
                combined.push(line.clone());

                processed_data.push(combined.clone());
                combined.clear();
            } else {
                combined.push(line.clone())
            }
        }
    }

    for processed in processed_data {
        extract_function_arm(processed);
    }
    // println!("{processed_data:#?}");
}

fn extract_function_arm(data: Vec<LineDescriptions>) {
    let mut opened_braces = 0;
    let mut stringified = String::new();

    for raw in data {
        stringified.push_str(&raw.to_string());
        // println!("{}", raw.to_string())
    }

    let start_index = stringified.find("{");
    if let Some(_) = start_index {
        println!(
            "{:#?}\n\n\n",
            LineDescriptions::to_struct(
                stringified[start_index.unwrap()..stringified.len()].to_string()
            )
        );
    }
}
