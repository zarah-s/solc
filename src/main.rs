use regex::Regex;
use std::{env, fs};

// Define the token types
#[derive(Debug)]
enum Token {
    UInt256,
    Identifier(String),
    Assign,
    IntegerLiteral(i32),
    SemiColon,

    OpenParen,
    CloseParen,
    StringLiteral,
    OpenArm,
    CloseArm,
    // Add more token types as needed
}

fn lex(input: &str) -> Vec<Token> {
    // Define regular expressions for tokens
    let uint256_regex = Regex::new(r"uint256").unwrap();
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
    let assign_regex = Regex::new(r"=").unwrap();
    let int_literal_regex = Regex::new(r"\d+").unwrap();
    let semi_colon_literal_regex = Regex::new(r";").unwrap();

    let mut tokens = Vec::new();
    let mut remaining_input = input;

    // println!("{remaining_input}");

    while !remaining_input.is_empty() {
        // Skip whitespaces
        // if let Some(whitespace_match) = Regex::new(r"\s+").unwrap().find(remaining_input) {
        //     let whitespace_end = whitespace_match.end();
        //     remaining_input = &remaining_input[whitespace_end..];
        //     println!("space here {:?} {remaining_input}", whitespace_match);
        //     continue;
        // }

        // Match uint256
        if let Some(uint256_match) = uint256_regex.find(remaining_input) {
            let uint256_end = uint256_match.end();
            tokens.push(Token::UInt256);
            remaining_input = &remaining_input[uint256_end..];
            continue;
        }

        // Match identifier
        if let Some(identifier_match) = identifier_regex.find(remaining_input) {
            let identifier_end = identifier_match.end();
            let identifier_str = &remaining_input[..identifier_end].trim();
            tokens.push(Token::Identifier(identifier_str.to_string()));
            remaining_input = &remaining_input[identifier_end..];
            continue;
        }

        //Match assignment operator
        if let Some(assign_match) = assign_regex.find(remaining_input) {
            let assign_end = assign_match.end();
            tokens.push(Token::Assign);
            remaining_input = &remaining_input[assign_end..];
            continue;
        }

        //Match SemiColon
        if let Some(assign_match) = semi_colon_literal_regex.find(remaining_input) {
            let assign_end = assign_match.end();
            tokens.push(Token::SemiColon);
            remaining_input = &remaining_input[assign_end..];
            continue;
        }

        //Match integer literal
        if let Some(int_match) = int_literal_regex.find(remaining_input) {
            let int_end = int_match.end();
            let int_str = &remaining_input[..int_end].trim();
            let int_value = int_str.parse::<i32>().unwrap();
            tokens.push(Token::IntegerLiteral(int_value));
            remaining_input = &remaining_input[int_end..];
            continue;
        }

        // Handle unrecognized characters
        panic!(
            "Unexpected character: {}",
            remaining_input.chars().next().unwrap()
        );
    }

    tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let scripts: &[String] = &args[1..];
    // let file = fs::read_to_string(&scripts[0]).expect("Could not read file");
    let input = "uint256 myNum = 5;";

    let tokens = lex(input.trim());
    println!("{tokens:?}");

    // let lines = file.lines();

    // let oi: Vec<_> = lines.filter(|f| !f.starts_with("//")).collect();
    // let mut opend_braces = 0;
    // let mut chars = String::new();
    // for line in oi.iter().enumerate() {
    //     for jj in line.1.chars() {
    //         if jj == '{' {
    //             opend_braces += 1;
    //         }
    //         if jj == '}' {
    //             opend_braces -= 1;
    //         }
    //         chars.push(jj);
    //     }
    // }
    // println!(" {chars:?} {opend_braces}")
}
