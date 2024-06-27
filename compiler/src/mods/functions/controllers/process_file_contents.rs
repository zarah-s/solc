use std::{io::ErrorKind, process};

use tokio::fs;

use crate::mods::types::{
    compiler_errors::{CompilerError, IOError},
    line_descriptors::LineDescriptions,
};

pub async fn process_file_contents(args: Vec<String>) -> Vec<LineDescriptions<String>> {
    /* CHECK FOR VALID ARGUMENTS */
    if args.len() < 2 {
        CompilerError::IOError(IOError::IOError("Missing file path")).throw();
    }

    /* VALIDATE FILE FORMAT */
    if args[1].split(".").last().unwrap() != "sol" {
        CompilerError::IOError(IOError::IOError("Expecting .sol file.")).throw();
    }

    /* READ FILE TO STRING */
    let file_contents = fs::read_to_string(&args[1])
        .await
        .unwrap_or_else(|err| match err.kind() {
            ErrorKind::NotFound => {
                CompilerError::IOError(IOError::FileNotFound("File not found")).throw();
                process::exit(1);
            }
            _ => panic!("{}", err),
        });
    let mut lines_descriptors: Vec<LineDescriptions<String>> = Vec::new();

    /* CREATE STRUCTURE WITH LINES */
    structure_to_line_descriptors(file_contents, &mut lines_descriptors);

    /* STRIP COMMENTS AND DOC STRINGS */
    let parsable_structure = strip_comments(lines_descriptors);

    parsable_structure
}

fn structure_to_line_descriptors(
    file_contents: String,
    lines_: &mut Vec<LineDescriptions<String>>,
) {
    for (index, content) in file_contents.lines().enumerate() {
        lines_.push(LineDescriptions {
            line: (index as i32) + 1,
            data: content.to_string(),
        })
    }
}

fn strip_comments(lines_: Vec<LineDescriptions<String>>) -> Vec<LineDescriptions<String>> {
    let mut stripped_inline_comments: Vec<LineDescriptions<String>> = Vec::new();

    let mut quote = String::new();

    /* STRIP COMMENTS AND DOC STRINGS */
    let mut terminated_doc_string = true;
    let mut opened_quote = false;
    for stripped_comment in lines_.iter() {
        let mut combined = String::new();
        let comment_index: Option<usize> = stripped_comment.data.find("//");
        if let Some(index_value) = comment_index {
            let string_data = stripped_comment.data[..index_value].trim().to_string();
            if !string_data.trim().is_empty() {
                stripped_inline_comments.push(LineDescriptions {
                    data: string_data.trim().to_string(),
                    ..*stripped_comment
                })
            }
        } else {
            for (i, _char) in stripped_comment.data.char_indices() {
                if _char == '\'' || _char == '"' {
                    if opened_quote && _char.to_string() == quote {
                        opened_quote = false;
                    } else {
                        quote = _char.to_string();
                        opened_quote = true;
                    }
                }
                if _char == '/' && !opened_quote {
                    if terminated_doc_string {
                        let inp = stripped_comment.data.chars().collect::<Vec<_>>();
                        let next_char = inp.get(i + 1);
                        if let Some(_next) = next_char {
                            if *_next == '*' {
                                terminated_doc_string = false;
                            }
                        }
                    } else {
                        if _char == '/' {
                            let inp = stripped_comment.data.chars().collect::<Vec<_>>();
                            let prev_char = inp.get(i - 1);
                            if let Some(_prev) = prev_char {
                                if *_prev == '*' {
                                    terminated_doc_string = true;
                                    continue;
                                }
                            }
                        }
                    }
                }
                if terminated_doc_string {
                    combined.push(_char);
                }
            }
            if !combined.trim().is_empty() {
                stripped_inline_comments.push(LineDescriptions {
                    data: combined.trim().to_string(),
                    ..*stripped_comment
                });
            }
        }
    }
    assert!(terminated_doc_string, "No correspinding end for doc string");
    stripped_inline_comments
}
