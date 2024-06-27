use crate::mods::constants::constants::{DATA_TYPES, INTEGER_SIZES, KEYWORDS, SYMBOLS};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Contract,
    Solidity,
    Library,
    Using,
    Abstract,
    Emit,
    Call,
    Import,
    From,
    Delegatecall,
    Payable,
    Indexed,
    Modifier,
    Interface,
    Revert,
    Space,
    Event,
    Ether,
    Wei,
    Bytes(Option<u16>),
    Assert,
    Require,
    Storage,
    Error,
    Override,
    Push,
    Pop,
    While,
    Delete,
    Enum,
    Immutable,
    Is,
    Mutable,
    Constant,
    Internal,
    External,
    Virtual,
    Calldata,
    New,
    Mapping,
    Msg,
    Pragma,
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
    Uint(Option<u16>),
    Gwei,
    Days,
    Weeks,
    Years,
    Receive,
    Fallback,
    Cron,
    Gasless,
    Int(Option<u16>),
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
    Gt,
    Lt,
    Dot,
    Equals,
    Bang,
    Modulu,
    SemiColon,
    Quotation,
    Coma,
    Or,
    And,
    Xor,
    Not,
    True,
    False,
}

pub trait TokenTrait {
    fn to_string(&self) -> String;
    fn tokenize(input: &str) -> Token;
    fn lex(input: &str) -> Vec<Token>;
}

pub trait VecExtension {
    fn to_string(&self) -> String;
    fn strip_spaces(&self) -> Self;
}

pub trait StringExtension {
    fn tokenize(&self) -> Token;
    fn lex(&self) -> Vec<Token>;
}

impl VecExtension for Vec<Token> {
    fn to_string(&self) -> String {
        let mut stringified = String::new();
        for _token in self.iter() {
            stringified.push_str(&_token.to_string())
        }

        stringified
    }

    fn strip_spaces(&self) -> Self {
        let mut result = Vec::new();
        for token in self.iter() {
            match token {
                Token::Space => {}
                _ => {
                    result.push(token.to_owned());
                }
            }
        }

        result
    }
}

impl VecExtension for Vec<&Token> {
    fn to_string(&self) -> String {
        let mut stringified = String::new();
        for _token in self.iter() {
            stringified.push_str(&_token.to_string())
        }

        stringified
    }
    fn strip_spaces(&self) -> Self {
        let mut result = Vec::new();
        for token in self.iter() {
            match token {
                Token::Space => {}
                _ => {
                    result.push(token.to_owned());
                }
            }
        }

        result
    }
}

impl StringExtension for String {
    fn tokenize(&self) -> Token {
        tokenize(&self)
    }

    fn lex(&self) -> Vec<Token> {
        lex(&self)
    }
}

impl StringExtension for &str {
    fn tokenize(&self) -> Token {
        tokenize(&self)
    }
    fn lex(&self) -> Vec<Token> {
        lex(&self)
    }
}

impl StringExtension for char {
    fn tokenize(&self) -> Token {
        tokenize(&self.to_string().as_str())
    }
    fn lex(&self) -> Vec<Token> {
        lex(&self.to_string().as_str())
    }
}

impl TokenTrait for Token {
    fn to_string(&self) -> String {
        detokenize(&self)
    }

    fn tokenize(input: &str) -> Token {
        tokenize(input)
    }

    fn lex(input: &str) -> Vec<Token> {
        lex(input)
    }
}

impl TokenTrait for &Token {
    fn to_string(&self) -> String {
        detokenize(&self)
    }

    fn tokenize(input: &str) -> Token {
        tokenize(input)
    }

    fn lex(input: &str) -> Vec<Token> {
        lex(input)
    }
}

fn detokenize(input: &Token) -> String {
    match input {
        Token::Contract => "contract".to_string(),
        Token::Emit => "emit".to_string(),
        Token::Import => "import".to_string(),
        Token::Pragma => "pragma".to_string(),
        Token::From => "from".to_string(),
        Token::Call => "call".to_string(),
        Token::Using => "using".to_string(),
        Token::Days => "days".to_string(),
        Token::Weeks => "weeks".to_string(),
        Token::Years => "years".to_string(),
        Token::Gwei => "gwei".to_string(),
        Token::Delegatecall => "delegatecall".to_string(),
        Token::Library => "library".to_string(),
        Token::Solidity => "solidity".to_string(),
        Token::Abstract => "abstract".to_string(),
        Token::Indexed => "indexed".to_string(),
        Token::Modifier => "modifier".to_string(),
        Token::Space => " ".to_string(),
        Token::Interface => "interface".to_string(),
        Token::Assert => "assert".to_string(),
        Token::Is => "is".to_string(),
        Token::Event => "event".to_string(),
        Token::Ether => "ether".to_string(),
        Token::Wei => "wei".to_string(),
        Token::Bytes(size) => {
            if let Some(_size) = size {
                format!("bytes{}", _size)
            } else {
                "bytes".to_string()
            }
        }
        Token::Revert => "revert".to_string(),
        Token::Storage => "storage".to_string(),
        Token::While => "while".to_string(),
        Token::True => "true".to_string(),
        Token::False => "false".to_string(),
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
        Token::Uint(size) => {
            if let Some(_size) = size {
                format!("uint{}", _size)
            } else {
                "uint".to_string()
            }
        }
        Token::Int(size) => {
            if let Some(_size) = size {
                format!("int{}", _size)
            } else {
                "int".to_string()
            }
        }

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
        Token::Gt => ">".to_string(),
        Token::Lt => "<".to_string(),
        Token::Dot => ".".to_string(),
        Token::Equals => "=".to_string(),
        Token::Bang => "!".to_string(),
        Token::Modulu => "%".to_string(),
        Token::SemiColon => ";".to_string(),
        Token::Quotation => "\"".to_string(),
        Token::Coma => ".to_string(),".to_string(),
        Token::Or => "|".to_string(),
        Token::And => "&".to_string(),
        Token::Not => "~".to_string(),
        Token::Xor => "^".to_string(),
        Token::Identifier(val) => val.to_string(),
    }
}

fn tokenize(input: &str) -> Token {
    match input {
        "revert" => Token::Revert,
        " " => Token::Space,
        "emit" => Token::Emit,
        "pragma" => Token::Pragma,
        "import" => Token::Import,
        "from" => Token::From,
        "using" => Token::Using,
        "abstract" => Token::Abstract,
        "library" => Token::Library,
        "call" => Token::Call,
        "delegatecall" => Token::Delegatecall,
        "modifier" => Token::Modifier,
        "assert" => Token::Assert,
        "indexed" => Token::Indexed,
        "wei" => Token::Wei,
        "gwei" => Token::Gwei,
        "days" => Token::Days,
        "weeks" => Token::Weeks,
        "years" => Token::Years,
        "solidity" => Token::Solidity,
        "interface" => Token::Interface,
        "ether" => Token::Ether,
        "event" => Token::Event,
        "while" => Token::While,
        "contract" => Token::Contract,
        "mapping" => Token::Mapping,
        "storage" => Token::Storage,
        "delete" => Token::Delete,
        "push" => Token::Push,
        "pop" => Token::Pop,
        "msg" => Token::Msg,
        "is" => Token::Is,
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
        "true" => Token::True,
        "false" => Token::False,
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
        ">" => Token::Gt,
        "<" => Token::Lt,
        "." => Token::Dot,
        "=" => Token::Equals,
        "!" => Token::Bang,
        "%" => Token::Modulu,
        ";" => Token::SemiColon,
        "'" => Token::Quotation,
        "\"" => Token::Quotation,
        "," => Token::Coma,
        "|" => Token::Or,
        "&" => Token::And,
        "^" => Token::Xor,
        "~" => Token::Not,

        _other => process_dyn(_other),
    }
}

fn process_dyn(input_val: &str) -> Token {
    if input_val.starts_with("bytes") {
        let chars = input_val.chars().collect::<Vec<_>>();
        let next = chars.get("bytes".len());
        if let Some(_next) = next {
            if _next.is_ascii_digit() {
                Token::Bytes(extract_size(input_val))
            } else {
                Token::Identifier(input_val.to_string())
            }
        } else {
            Token::Bytes(extract_size(input_val))
        }
    } else if input_val.starts_with("uint") {
        let chars = input_val.chars().collect::<Vec<_>>();
        let next = chars.get("uint".len());
        if let Some(_next) = next {
            if _next.is_ascii_digit() {
                let size = extract_size(input_val);
                if let Some(_size) = size {
                    if INTEGER_SIZES.contains(&_size) {
                        return Token::Uint(size);
                    } else {
                        return Token::Identifier(input_val.to_string());
                    }
                }
                Token::Uint(size)
            } else {
                Token::Identifier(input_val.to_string())
            }
        } else {
            Token::Uint(extract_size(input_val))
        }
    } else if input_val.starts_with("int") {
        let chars = input_val.chars().collect::<Vec<_>>();
        let next = chars.get("int".len());
        if let Some(_next) = next {
            if _next.is_ascii_digit() {
                let size = extract_size(input_val);
                if let Some(_size) = size {
                    if INTEGER_SIZES.contains(&_size) {
                        return Token::Int(size);
                    } else {
                        return Token::Identifier(input_val.to_string());
                    }
                }
                Token::Int(size)
            } else {
                Token::Identifier(input_val.to_string())
            }
        } else {
            Token::Int(extract_size(input_val))
        }
    } else {
        Token::Identifier(input_val.to_string())
    }
}

fn extract_size(s: &str) -> Option<u16> {
    let number_string: String = s.chars().filter(|c| c.is_digit(10)).collect();

    if number_string.is_empty() {
        None
    } else {
        number_string.parse::<u16>().ok()
    }
}

fn lex(input: &str) -> Vec<Token> {
    let mut combined_strings: Vec<String> = Vec::new();
    let mut combined_char = String::new();
    let mut lexems: Vec<Token> = Vec::new();
    let mut quote = String::new();

    let mut opened_quote = false;
    let mut data_type_pause = false;

    for (index, character) in input.trim().char_indices() {
        if character.is_whitespace() {
            if !combined_char.trim().is_empty() {
                if opened_quote {
                    combined_char.push(character)
                } else {
                    combined_strings.push(combined_char.trim().to_string());
                    combined_char.clear();
                }
            } else {
                let chars = input.trim().chars().collect::<Vec<_>>();
                let next = chars.get(index + 1);
                if let Some(_next) = next {
                    if !_next.is_whitespace() {
                        if opened_quote {
                            combined_char.push(character)
                        } else {
                            assert!(combined_char.trim().is_empty(), "Missing quotation");
                            combined_strings.push(character.to_string());
                        }
                        continue;
                    }
                }
                if opened_quote {
                    combined_char.push(character);
                } else {
                    continue;
                }
            }
        } else if character == '"' || character == '\'' {
            combined_char.push(character);
            if opened_quote && quote == character.to_string() {
                opened_quote = false;
                quote.clear();
                combined_strings.push(combined_char.trim().to_string());
                combined_char.clear();
            } else {
                opened_quote = true;
                quote = character.to_string();
            }
        } else if SYMBOLS.contains(&character) {
            if data_type_pause {
                combined_strings.push(combined_char.trim().to_string());
                combined_char.clear();
                combined_strings.push(character.to_string());

                data_type_pause = false;
            } else {
                if opened_quote {
                    combined_char.push(character);
                } else {
                    if combined_char.trim().is_empty() {
                        combined_strings.push(character.to_string())
                    } else {
                        combined_strings.push(combined_char.trim().to_string());
                        combined_char.clear();
                        combined_strings.push(character.to_string().trim().to_string())
                    }
                }
            }
        } else if DATA_TYPES.contains(&format!("{}{}", combined_char.trim(), character).as_str()) {
            let chars = input.trim().chars().collect::<Vec<_>>();
            let next = chars.get(index + 1);
            if let Some(_next) = next {
                if SYMBOLS.contains(_next) || _next.is_whitespace() {
                    combined_strings.push(format!("{}{}", combined_char.trim(), character));
                    combined_char.clear();
                } else {
                    data_type_pause = true;
                    combined_char.push(character)
                }
            } else {
                combined_strings.push(format!("{}{}", combined_char.trim(), character));
                combined_char.clear();
            }
        } else if KEYWORDS.contains(&format!("{}{}", combined_char.trim(), character).as_str()) {
            let chars = input.trim().chars().collect::<Vec<_>>();
            let next = chars.get(index + 1);
            if let Some(_next) = next {
                if !_next.is_whitespace() && _next.is_alphabetic() {
                    combined_char.push(character);
                } else {
                    combined_strings.push(format!("{}{}", combined_char.trim(), character));
                    combined_char.clear();
                }
            } else {
                combined_strings.push(format!("{}{}", combined_char.trim(), character));
                combined_char.clear();
            }
        } else {
            combined_char.push(character);
            if index == input.trim().len() - 1 {
                combined_strings.push(combined_char.trim().to_string());
                combined_char.clear();
            }
        }
    }

    assert!(combined_char.is_empty(), "Internal Error");

    for combined_string in combined_strings {
        lexems.push(combined_string.tokenize())
    }

    lexems
}

#[derive(PartialEq)]
pub enum Context {
    Import,
    Interface,
    Library,
    Header,
    Contract,
    Error,
    None,
}
