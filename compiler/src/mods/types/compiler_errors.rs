#[derive(Debug)]
pub enum CompilerError<'a> {
    LexicalError(LexicalError),
    SyntaxError(SyntaxError<'a>),
    SemanticError(SemanticError<'a>),
    IOError(IOError<'a>),
    InternalError(&'a str),
}

#[derive(Debug)]
pub enum LexicalError {
    InvalidCharacter(char),
    UnterminatedString,
    UnexpectedEndOfFile,
}

#[derive(Debug)]
pub enum SyntaxError<'a> {
    UnexpectedToken(&'a str),
    MissingToken(&'a str),
    SyntaxError(&'a str),
}

#[derive(Debug)]
pub enum SemanticError<'a> {
    UndefinedVariable(&'a str),
    Redeclaration(&'a str),
    TypeMismatch(&'a str),
    UndefinedFunction(&'a str),
    InvalidOperation(&'a str),
}

#[derive(Debug)]
pub enum IOError<'a> {
    FileNotFound(&'a str),
    IOError(&'a str),
}

impl<'a> CompilerError<'a> {
    pub fn throw(&self) {
        match &self {
            CompilerError::LexicalError(lex_error) => {
                panic!("Lexical error: {:?}", lex_error);
            }
            CompilerError::SyntaxError(syntax_error) => {
                panic!("Syntax error: {:?}", syntax_error);
            }
            CompilerError::SemanticError(semantic_error) => {
                panic!("Semantic error: {:?}", semantic_error);
            }
            CompilerError::IOError(io_error) => {
                panic!("IO error: {:?}", io_error);
            }
            CompilerError::InternalError(message) => {
                panic!("Internal error: {}", message);
            }
        }
    }

    pub fn throw_with_file_info(&self, file: &str, line: i32) {
        match &self {
            CompilerError::LexicalError(lex_error) => {
                panic!(
                    "Lexical error: {:?}\n\x1b[4m{file} {line}\x1b[24m",
                    lex_error
                );
            }
            CompilerError::SyntaxError(syntax_error) => {
                panic!(
                    "Syntax error: {:?}\n\x1b[4m{file} {line}\x1b[24m",
                    syntax_error
                );
            }
            CompilerError::SemanticError(semantic_error) => {
                panic!(
                    "Semantic error: {:?}\n\x1b[4m{file} {line}\x1b[24m",
                    semantic_error
                );
            }
            CompilerError::IOError(io_error) => {
                panic!("IO error: {:?}\n\x1b[4m{file} {line}\x1b[24m", io_error);
            }
            CompilerError::InternalError(message) => {
                panic!("Internal error: {}\n\x1b[4m{file} {line}\x1b[24m", message);
            }
        }
    }
}
