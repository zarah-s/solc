pub mod helpers {
    use crate::{LineDescriptions, Token};
    use eval::eval;
    use std::process;
    pub fn print_error(msg: &str) {
        eprintln!("ERROR: {}", msg);
        process::exit(1);
    }

    pub fn lex_to_token(input: &str) -> Token {
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

    pub fn detokenize(input: &Token) -> String {
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

    pub fn extract_data_location_from_token(token: &Token) -> Option<Token> {
        match token {
            Token::Memory => Some(Token::Memory),
            Token::Calldata => Some(Token::Calldata),
            _ => None,
        }
    }

    pub fn extract_integer_types_from_token(token: &Token) -> Option<Token> {
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

    pub fn extract_data_types_from_token(token: &Token) -> Option<Token> {
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
            Token::String => Some(Token::String),
            Token::Bool => Some(Token::Bool),
            Token::Address => Some(Token::Address),
            _ => None,
        }
    }

    pub fn validate_expression(expression: &String, text: LineDescriptions) -> Option<String> {
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

                    if let Token::Identifier(_power_identifier) =
                        &raised_expr[raised_expr.len() - 1]
                    {
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
}
