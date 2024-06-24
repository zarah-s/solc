use std::{io::ErrorKind, process};

use tokio::fs;

use crate::mods::types::{
    compiler_errors::{CompilerError, IOError},
    line_descriptors::LineDescriptions,
};

pub async fn process_file_contents(args: Vec<String>) -> Vec<LineDescriptions> {
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
    let mut lines_descriptors: Vec<LineDescriptions> = Vec::new();

    /* CREATE STRUCTURE WITH LINES */
    structure_to_line_descriptors(file_contents, &mut lines_descriptors);

    /* STRIP COMMENTS AND DOC STRINGS */
    let structured_stripped_compilable_contents = strip_comments(lines_descriptors);

    structured_stripped_compilable_contents
}

fn structure_to_line_descriptors(file_contents: String, lines_: &mut Vec<LineDescriptions>) {
    for (index, content) in file_contents.lines().enumerate() {
        lines_.push(LineDescriptions {
            line: (index as i32) + 1,
            text: content.to_string(),
        })
    }
}

fn strip_comments(lines_: Vec<LineDescriptions>) -> Vec<LineDescriptions> {
    let mut stripped_inline_comments: Vec<LineDescriptions> = Vec::new();

    let mut quote = String::new();

    /* STRIP COMMENTS AND DOC STRINGS */
    let mut terminated = true;
    let mut opened_quote = false;
    for stripped_comment in lines_.iter() {
        let mut combined = String::new();
        let comment_index: Option<usize> = stripped_comment.text.find("//");
        if let Some(index_value) = comment_index {
            let string_data = stripped_comment.text[..index_value].trim().to_string();
            if !string_data.trim().is_empty() {
                stripped_inline_comments.push(LineDescriptions {
                    text: string_data.trim().to_string(),
                    ..*stripped_comment
                })
            }
        } else {
            for (i, __dd) in stripped_comment.text.char_indices() {
                if __dd == '\'' || __dd == '"' {
                    if opened_quote && __dd.to_string() == quote {
                        opened_quote = false;
                    } else {
                        quote = __dd.to_string();
                        opened_quote = true;
                    }
                }
                if __dd == '/' && !opened_quote {
                    if terminated {
                        let inp = stripped_comment.text.chars().collect::<Vec<_>>();
                        let next_char = inp.get(i + 1);
                        if let Some(_next) = next_char {
                            if *_next == '*' {
                                terminated = false;
                            }
                        }
                    } else {
                        if __dd == '/' {
                            let inp = stripped_comment.text.chars().collect::<Vec<_>>();
                            let prev_char = inp.get(i - 1);
                            if let Some(_prev) = prev_char {
                                if *_prev == '*' {
                                    terminated = true;
                                    continue;
                                }
                            }
                        }
                    }
                }
                if terminated {
                    combined.push(__dd);
                }
            }
            if !combined.trim().is_empty() {
                stripped_inline_comments.push(LineDescriptions {
                    text: combined.trim().to_string(),
                    ..*stripped_comment
                });
            }
        }
    }
    stripped_inline_comments
}
