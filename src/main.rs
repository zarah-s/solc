use regex::Regex;
use std::{env, fs, process};

#[derive(Debug)]
enum Token {
    VariableIdentifier(String, String, String, String),
    //DATATYPE, VISIBILITY, NAME, VALUE;
}

const DATA_TYPES: [&str; 10] = [
    "uint8", "uint16", "uint32", "uint256", "int8", "int16", "int32", "int256", "bool", "string",
];
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: Compiler needs a file path");
        process::exit(1);
    }

    if args[1].split(".").last().unwrap() != "sol" {
        eprintln!("ERROR: Invalid file format");
        process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        eprintln!("ERROR: Error opening file <{}>. {err}", args[1]);
        process::exit(1);
    });

    let mut program_lines: Vec<&str> = vec![];

    for line in file_content
        .lines()
        .filter(|ft| !ft.trim().starts_with("//") && !ft.is_empty())
    {
        let check_double: Vec<&str> = line.trim().split(";").collect();

        for db_lines in check_double {
            // println!("{db_lines}")
            if !db_lines.trim().is_empty() {
                program_lines.push(db_lines.trim());
            }
        }

        // check_double.map(|e| if !e.trim().is_empty() {})
    }

    // println!("{program_lines:?}");
    let mut tokens = Vec::new();

    for pr_line in &program_lines {
        // println!("{pr_line}")
        // lex(pr_line);
        tokens.push(variable_lexing(&pr_line));
        // let token: Token = variable_lexing(&pr_line);
    }
    println!("{:?}", tokens);

    // println!("{program_lines:?} {}", program_lines[3]);
}

fn variable_lexing(input: &str) -> Token {
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
    let int_literal_regex = Regex::new(r"\d+").unwrap();
    let bool_literal_regex = Regex::new(r"\b(true|false)\b").unwrap();
    let string_literal_regex = Regex::new(r#""([^"]*)""#).unwrap();
    let line_arm: Vec<&str> = input.split(" ").collect();

    let mut data_type: Option<String> = None;
    let mut visibility: Option<String> = None;
    let mut name: Option<String> = None;
    let mut value: Option<String> = None;

    if !DATA_TYPES.contains(&line_arm[0]) {
        eprintln!("ERROR: Unidentified data type {}", line_arm[0]);
        process::exit(1);
    } else {
        let blah = line_arm[1..].join(" ");
        // println!("{} {}", line_arm.join(" "), line_arm.len());
        data_type = Some(line_arm[0].to_string());
        if let Some(identifier_match) = &identifier_regex.find(&blah) {
            if identifier_match.as_str() == "public" {
                if let Some(blah_match) = identifier_regex.find(&blah[identifier_match.end()..]) {
                    name = Some(blah_match.as_str().to_string())
                }
            } else {
                name = Some(identifier_match.as_str().to_string())
            }
        }

        if let Some(visibility_match) = Regex::new(r"public").unwrap().find(&blah) {
            visibility = Some(visibility_match.as_str().to_string());
        }

        if let Some(int_match) = int_literal_regex.find(&blah) {
            value = Some(int_match.as_str().to_string());
        }

        if let Some(string_match) = string_literal_regex.find(&blah) {
            value = Some(string_match.as_str().to_string());
        }

        if let Some(bool_match) = bool_literal_regex.find(&blah) {
            value = Some(bool_match.as_str().to_string());
        }

        if let None = data_type {
            print_error("Invalid syntax");
        }

        if let None = visibility {
            visibility = Some("private".to_string());
        }

        if let None = name {
            print_error("Variable name required");
        }

        if let None = value {
            print_error("Missing value for variable")
        }

        let token = Token::VariableIdentifier(
            data_type.unwrap(),
            visibility.unwrap(),
            name.unwrap(),
            value.unwrap(),
        );
        token
    }
}

fn print_error(msg: &str) {
    eprintln!("ERROR: {msg}");
    process::exit(1);
}
