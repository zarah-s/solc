use std::{env, fs, process};

use regex::Regex;

const DATA_TYPES: [&str; 14] = [
    "uint8", "uint16", "uint", "uint32", "bytes1", "uint256", "int", "int8", "int16", "int32",
    "int256", "bool", "string", "address",
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

#[derive(Debug)]
enum OpenedBraceType {
    None,
    Struct,
    Callback,
    Function,
    Contract,
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

    let mut global_variables_expression: Vec<LineDescriptions> = Vec::new();
    let mut struct_variants: Vec<StructIdentifier> = Vec::new();
    let mut custom_data_types: Vec<String> = Vec::new();
    let mut opened_braces = 0;
    let mut opened_brace_type = OpenedBraceType::Contract;

    let mut processing_struct_name: Option<String> = None;
    let mut processing_struct_types: Vec<StructTypes> = Vec::new();

    // for structured in structured_stripped_compilable_contents {
    //     if structured.text.contains("{") {
    //         opened_braces += 1;
    //     }

    //     if structured.text.contains("}") {
    //         if opened_braces - 1 == 1 {
    //             if let OpenedBraceType::Struct = opened_brace_type {
    //                 struct_variants.push(StructIdentifier {
    //                     identifier: processing_struct_name.clone().unwrap(),
    //                     types: processing_struct_types.clone(),
    //                 });
    //                 processing_struct_name = None;
    //                 processing_struct_types.clear();
    //             }
    //             opened_brace_type = OpenedBraceType::Contract;
    //         }
    //         opened_braces -= 1;
    //     }

    //     if let OpenedBraceType::Contract = opened_brace_type {
    //         for inline in structured.clone().text.split(";") {
    //             if !inline.is_empty() {
    //                 let text: Vec<&str> = inline.split(" ").collect();

    //                 if DATA_TYPES.contains(text.first().unwrap())
    //                     || custom_data_types.contains(&text.first().unwrap().to_string())
    //                 {
    //                     global_variables_expression.push(LineDescriptions {
    //                         text: inline.to_string(),
    //                         ..structured.clone()
    //                     });
    //                 } else {
    //                     if text.first().unwrap().contains("[]") {
    //                         let start_index = text.first().unwrap().find("[");
    //                         let extracted_var = &text.first().unwrap()[..start_index.unwrap()];
    //                         if DATA_TYPES.contains(&extracted_var) {
    //                             global_variables_expression.push(LineDescriptions {
    //                                 text: inline.to_string(),
    //                                 ..structured.clone()
    //                             });
    //                             // global_variables_expression.push(structured.clone());
    //                         } else {
    //                             if custom_data_types.contains(&extracted_var.to_string()) {
    //                                 global_variables_expression.push(LineDescriptions {
    //                                     text: inline.to_string(),
    //                                     ..structured.clone()
    //                                 });
    //                             } else {
    //                                 print_error(&format!(
    //                                     "Unidentified argument \"{}\" at line {}",
    //                                     text.first().unwrap(),
    //                                     structured.line,
    //                                 ));
    //                             }
    //                         }
    //                     } else {
    //                         if !KEYWORDS.contains(text.first().unwrap())
    //                             && !SYMBOLS.contains(text.first().unwrap())
    //                         {
    //                             print_error(&format!(
    //                                 "Unidentified argument \"{}\" at line {}",
    //                                 text.first().unwrap(),
    //                                 structured.line
    //                             ));
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         if structured.text.starts_with("struct") {
    //             opened_brace_type = OpenedBraceType::Struct;
    //             let struct_definition: Vec<&str> = structured.text.split(" ").collect();
    //             if struct_definition.len() != 3 {
    //                 print_error(&format!(
    //                     "Unidentified argument \"{}\" at line {}",
    //                     structured.text, structured.line,
    //                 ));
    //             } else {
    //                 processing_struct_name = Some(struct_definition[1].to_string());
    //                 custom_data_types.push(struct_definition[1].to_string());
    //             }
    //         } else {
    //             if let OpenedBraceType::Struct = opened_brace_type {
    //                 if !structured.text.ends_with(";") && structured.text != "}" {
    //                     print_error(&format!(
    //                         "Missing argument \";\" on line {} ",
    //                         structured.line,
    //                     ));
    //                 } else {
    //                     let types_def: Vec<&str> = structured.text.split(" ").collect();
    //                     if types_def.len() != 2 {
    //                         print_error(&format!(
    //                             "Unprocessible entity \"{}\" at line {}",
    //                             structured.text, structured.line,
    //                         ));
    //                     } else {
    //                         let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();

    //                         if !DATA_TYPES.contains(&types_def[0]) {
    //                             if types_def[0].contains("[]") {
    //                                 let start_index = types_def[0].find("[");
    //                                 let extracted_var = &types_def[0][..start_index.unwrap()];
    //                                 if DATA_TYPES.contains(&extracted_var) {
    //                                     if let None = identifier_regex.find(&types_def[1]) {
    //                                         print_error(&format!(
    //                                             "Unprocessible entity \"{}\" on line {}",
    //                                             structured.text, structured.line,
    //                                         ));
    //                                     } else {
    //                                         processing_struct_types.push(StructTypes::Type(
    //                                             types_def[0].to_string(),
    //                                             types_def[1].to_string(),
    //                                         ));
    //                                     }
    //                                 } else {
    //                                     print_error(&format!(
    //                                         "Unidentified  argument \"{}\" on line {}",
    //                                         structured.text, structured.line,
    //                                     ));
    //                                 }
    //                             } else {
    //                                 print_error(&format!(
    //                                     "Unidentified  argument \"{}\" on line {}",
    //                                     structured.text, structured.line,
    //                                 ));
    //                             }
    //                         } else {
    //                             if let None = identifier_regex.find(&types_def[1]) {
    //                                 print_error(&format!(
    //                                     "Unprocessible entity \"{}\" on line {}",
    //                                     structured.text, structured.line,
    //                                 ));
    //                             } else {
    //                                 processing_struct_types.push(StructTypes::Type(
    //                                     types_def[0].to_string(),
    //                                     types_def[1].to_string(),
    //                                 ));
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         if structured.text.starts_with("function") {
    //             opened_brace_type = OpenedBraceType::Function;
    //         }
    //     }
    // }

    println!(
        "{:#?} {:#?} {:#?}",
        struct_variants, global_variables_expression, custom_data_types
    )
}

fn print_error(msg: &str) {
    eprintln!("ERROR: {}", msg);
    process::exit(1);
}
