use crate::mods::{
    functions::controllers::process_file_contents::process_file_contents,
    types::{
        compiler_errors::{CompilerError, SyntaxError},
        line_descriptors::{LineDescriptions, StringDescriptor},
        token::{Context, Token, TokenTrait, VecExtension},
    },
};

pub async fn compile_source_code(args: Vec<String>) {
    let parsable_structure = process_file_contents(args).await;
    let mut imports: Vec<Vec<LineDescriptions<Vec<Token>>>> = Vec::new();
    let mut libraries: Vec<Vec<LineDescriptions<Vec<Token>>>> = Vec::new();
    let mut interfaces: Vec<Vec<LineDescriptions<Vec<Token>>>> = Vec::new();
    let mut contracts: Vec<Vec<LineDescriptions<Vec<Token>>>> = Vec::new();
    let mut custom_errors: Vec<Vec<LineDescriptions<Vec<Token>>>> = Vec::new();
    seperate_variants(
        parsable_structure,
        &mut imports,
        &mut interfaces,
        &mut contracts,
        &mut libraries,
        &mut custom_errors,
    );
    println!("{:#?}", imports);
}

fn seperate_variants(
    parsable_structure: Vec<LineDescriptions<String>>,
    imports: &mut Vec<Vec<LineDescriptions<Vec<Token>>>>,
    interfaces: &mut Vec<Vec<LineDescriptions<Vec<Token>>>>,
    contracts: &mut Vec<Vec<LineDescriptions<Vec<Token>>>>,
    libraries: &mut Vec<Vec<LineDescriptions<Vec<Token>>>>,
    custom_errors: &mut Vec<Vec<LineDescriptions<Vec<Token>>>>,
) {
    let mut is_import_brace = false;
    let mut opened_braces_count = 0;
    let mut tokens: Vec<Token> = Vec::new();

    let mut combined: Vec<LineDescriptions<Vec<Token>>> = Vec::new();
    let mut context = Context::None;
    for (parent_index, line_desc) in parsable_structure.iter().enumerate() {
        let lexems = line_desc.lex();
        // println!("{:?}", lexems);
        for (index, token) in lexems.data.iter().enumerate() {
            tokens.push(token.clone());
            match token {
                Token::Pragma => {
                    if parent_index > 0 {
                        validate_clash(context, &tokens, &parsable_structure.get(parent_index - 1));
                    }
                    context = Context::Header;
                }
                Token::Error => {
                    if parent_index > 0 {
                        validate_clash(context, &tokens, &parsable_structure.get(parent_index - 1));
                    }

                    context = Context::Error;
                }
                Token::Abstract => {
                    if parent_index > 0 {
                        validate_clash(context, &tokens, &parsable_structure.get(parent_index - 1));
                    }

                    context = Context::Contract;
                }
                Token::Library => {
                    if parent_index > 0 {
                        validate_clash(context, &tokens, &parsable_structure.get(parent_index - 1));
                    }

                    context = Context::Library;
                }
                Token::Import => {
                    if parent_index > 0 {
                        validate_clash(context, &tokens, &parsable_structure.get(parent_index - 1));
                    }

                    context = Context::Import;
                }

                Token::Interface => {
                    if parent_index > 0 {
                        validate_clash(context, &tokens, &parsable_structure.get(parent_index - 1));
                    }

                    context = Context::Interface;
                }
                Token::Contract => {
                    if let Context::None = context {
                    } else {
                        if !tokens.is_empty() {
                            if tokens.strip_spaces()[0] != Token::Abstract {
                                println!("{:?}", tokens);
                                panic!("contr")
                            }
                        }
                    }
                    context = Context::Contract;
                }

                Token::SemiColon => {
                    if opened_braces_count == 0 {
                        match context {
                            Context::Import => {
                                combined.push(LineDescriptions {
                                    data: tokens.clone(),
                                    line: lexems.line,
                                });
                                tokens.clear();
                                imports.push(combined.clone());
                                combined.clear();
                            }
                            Context::Header => {
                                tokens.clear();
                            }

                            Context::Error => {
                                combined.push(LineDescriptions {
                                    data: tokens.clone(),
                                    line: lexems.line,
                                });
                                tokens.clear();
                                custom_errors.push(combined.clone());
                                combined.clear();
                            }

                            _ => {
                                CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                                    &token.to_string(),
                                ))
                                .throw_with_file_info("Contract.sol", lexems.line);
                            }
                        }
                        context = Context::None;
                    }
                }

                Token::OpenBraces => {
                    if index > 0 {
                        let stripped = lexems.data.strip_spaces();
                        let prev = stripped.get(index - 2);
                        if prev.is_some() && *prev.unwrap() == Token::Import {
                            is_import_brace = true;
                        } else {
                            opened_braces_count += 1;
                        }
                    } else {
                        CompilerError::SyntaxError(SyntaxError::UnexpectedToken("{"))
                            .throw_with_file_info("Contract.sol", lexems.line);
                    }
                }
                Token::CloseBraces => {
                    if !is_import_brace {
                        opened_braces_count -= 1;
                        if opened_braces_count == 0 {
                            match context {
                                Context::Library => {
                                    combined.push(LineDescriptions {
                                        data: tokens.clone(),
                                        line: lexems.line,
                                    });
                                    tokens.clear();

                                    libraries.push(combined.clone());
                                    combined.clear();
                                }
                                Context::Interface => {
                                    combined.push(LineDescriptions {
                                        data: tokens.clone(),
                                        line: lexems.line,
                                    });
                                    tokens.clear();

                                    interfaces.push(combined.clone());
                                    combined.clear();
                                }

                                Context::Contract => {
                                    combined.push(LineDescriptions {
                                        data: tokens.clone(),
                                        line: lexems.line,
                                    });
                                    tokens.clear();

                                    contracts.push(combined.clone());
                                    combined.clear();
                                }
                                _ => {}
                            }
                            context = Context::None;
                        }
                    } else {
                        is_import_brace = false;
                    }
                }
                _ => {}
            }

            if let Context::None = context {
                if !tokens.strip_spaces().is_empty() {
                    CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                        &tokens.strip_spaces()[0].to_string(),
                    ))
                    .throw_with_file_info("Contract.sol", lexems.line);
                }
            }
        }

        if !tokens.is_empty() {
            combined.push(LineDescriptions {
                line: lexems.line,
                data: tokens.clone(),
            });
            tokens.clear();
        }
    }
}

/* VALIDATES CLASH DUE TO MISSING TOKEN E.G ";" OR "}" */
fn validate_clash(
    context: Context,
    tokens: &Vec<Token>,
    lexems: &Option<&LineDescriptions<String>>,
) {
    if let Some(_lexems) = lexems {
        if context != Context::None && !tokens.is_empty() {
            CompilerError::SyntaxError(SyntaxError::MissingToken(match context {
                Context::Contract | Context::Interface | Context::Library => "}",
                _ => ";",
            }))
            .throw_with_file_info("Contract.sol", _lexems.lex().line);
        }
    } else {
        CompilerError::InternalError("Unprocessible entity").throw();
    }
}
