use regex::Regex;
use std::{env, fs, process};

#[derive(Debug)]
#[allow(dead_code)]
enum Scope {
    Global,
    Functional(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Argument {
    Params(String, String),
    //DATATYPE, NAME
}

#[derive(Debug)]
#[allow(dead_code)]

struct StructIdentifier {
    identifier: String,
    types: Vec<Argument>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct FunctionIdentifier {
    name: String,
    arguments: Option<Vec<Argument>>,
    visibility: String,
    view: Option<String>,
    return_type: Option<String>,
    gasless: bool,
    payable: bool,

    arms: Vec<Token>,
}

#[derive(Clone, Copy)]

enum BraceType {
    Function,
    Struct,
    None,
}

enum OpenedBraces {
    Value(BraceType, i8),
}

impl FunctionIdentifier {
    pub fn new(
        name: String,
        visibility: String,
        view: Option<String>,
        arms: Vec<Token>,
        return_type: Option<String>,
        gasless: bool,
        payable: bool,
        arguments: Option<Vec<Argument>>,
    ) -> Self {
        Self {
            name,
            visibility,
            view,
            return_type,
            gasless,
            arms,
            payable,
            arguments,
        }
    }
}

#[derive(Debug)]
enum Token {
    VariableIdentifier(String, String, String, Option<String>, Scope),
    //DATATYPE, VISIBILITY, NAME, VALUE;
    FunctionIdentifier(FunctionIdentifier),
    Require(String, String),
    Struct(StructIdentifier),
    Assignment(String, String),
    StructVariableIdentifier(String, String, Vec<Argument>),
}

const DATA_TYPES: [&str; 11] = [
    "uint8", "uint16", "uint32", "uint256", "int8", "int16", "int32", "int256", "bool", "string",
    "address",
];
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_error("ERROR: Compiler needs a file path");
    }

    if args[1].split(".").last().unwrap() != "sol" {
        print_error("ERROR: Invalid file format");
    }

    let file_content = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        print_error(&format!("ERROR: Error opening file <{}>. {err}", args[1]));
        process::exit(1);
    });

    let mut program_lines: Vec<&str> = Vec::new();
    let mut custom_types: Vec<&str> = Vec::new();
    let mut func_expr: Vec<Vec<&str>> = Vec::new();
    let mut struct_expr: Vec<Vec<&str>> = Vec::new();
    let mut opened_braces: OpenedBraces = OpenedBraces::Value(BraceType::None, 0);

    for line in file_content
        .lines()
        .filter(|ft| !ft.trim().starts_with("//") && !ft.is_empty())
    {
        let check_double: Vec<&str> = line.trim().split(";").collect();

        if line.trim().starts_with("function") || line.trim().starts_with("struct") {
            if line.trim().ends_with("{") {
                let braces_num = match opened_braces {
                    OpenedBraces::Value(_, num) => num,
                };
                let func_start = line.trim().find("function");
                if let None = func_start {
                    opened_braces = OpenedBraces::Value(BraceType::Struct, braces_num + 1);
                } else {
                    opened_braces = OpenedBraces::Value(BraceType::Function, braces_num + 1);
                }
            }
        }
        let (brace_num, brace_type) = match opened_braces {
            OpenedBraces::Value(br_type, num) => (num, br_type),
        };
        if brace_num == 0 {
            for db_lines in check_double {
                if !db_lines.trim().is_empty() {
                    program_lines.push(db_lines.trim());
                }
            }
        }

        if brace_num > 0 {
            if let BraceType::Function = brace_type {
                if func_expr.is_empty() {
                    func_expr.push(vec![line.trim()])
                } else {
                    if func_expr[func_expr.len() - 1].ends_with(&["}"]) {
                        func_expr.push(vec![line.trim()])
                    } else {
                        let last_index = func_expr.len() - 1;
                        func_expr[last_index].push(line.trim());
                    }
                }
            }

            if let BraceType::Struct = brace_type {
                if struct_expr.is_empty() {
                    struct_expr.push(vec![line.trim()]);
                } else {
                    if struct_expr[struct_expr.len() - 1].ends_with(&["}"]) {
                        struct_expr.push(vec![line.trim()])
                    } else {
                        let last_index = struct_expr.len() - 1;
                        struct_expr[last_index].push(line.trim());
                    }
                }
            }
            if line.trim().ends_with("}") {
                let braces_num = match opened_braces {
                    OpenedBraces::Value(_, num) => num,
                };
                opened_braces = OpenedBraces::Value(brace_type, braces_num - 1);
            }
        }
    }

    let mut tokens = Vec::new();

    for custom_type in &struct_expr {
        let stripped_type = &custom_type[0].trim()[6..custom_type[0].len() - 1].trim();
        custom_types.push(stripped_type)
    }

    for str_expr in struct_expr {
        tokens.push(struct_lexing(str_expr, &custom_types))
    }

    for pr_line in &program_lines {
        if !pr_line.starts_with("function") {
            tokens.push(variable_lexing(
                &pr_line,
                Scope::Global,
                None,
                &custom_types,
            ));
        }
    }

    for expr in func_expr {
        let tokenized_expression = function_lexing(expr, &custom_types);
        tokens.push(tokenized_expression);
    }

    println!("{:#?}", tokens);
}

fn struct_lexing(input: Vec<&str>, custom_types: &Vec<&str>) -> Token {
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();

    let mut identifier: Option<&str> = None;
    let mut types: Vec<Argument> = Vec::new();

    if let Some(identifier_match) = identifier_regex.find(&input[0]["struct".len()..]) {
        identifier = Some(identifier_match.as_str());
    } else {
        print_error(&format!("Missing Struct name at {}", &input[0]))
    }

    let types_strip = &input[1..&input.len() - 1];

    for str_type in types_strip {
        for db_line in str_type.split(";") {
            if !db_line.is_empty() {
                let args: Vec<&str> = db_line.split(" ").collect();
                if args.len() != 2 || args[1].is_empty() {
                    print_error(&format!(
                        "Invalid value in \"struct {}\" {}",
                        identifier.unwrap(),
                        str_type
                    ));
                } else {
                    if !DATA_TYPES.contains(&args[0]) {
                        if args[0].contains("[]") {
                            if !DATA_TYPES.contains(&&args[0][..args[0].find("[").unwrap()])
                                && !custom_types.contains(&&args[0][..args[0].find("[").unwrap()])
                            {
                                print_error(&format!("ERROR: Unidentified data type {}", args[0]));
                            } else {
                                types.push(Argument::Params(
                                    args[0].to_string(),
                                    args[1].to_string(),
                                ))
                            }
                        }
                    } else {
                        types.push(Argument::Params(args[0].to_string(), args[1].to_string()))
                    }
                }
            }
        }
    }
    Token::Struct(StructIdentifier {
        identifier: identifier.unwrap().to_string(),
        types,
    })
}

fn variable_lexing(
    input: &str,
    scope: Scope,
    custum_data_type: Option<bool>,
    custom_types: &Vec<&str>,
) -> Token {
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
    // let int_literal_regex = Regex::new(r"\d+").unwrap();
    // let bool_literal_regex = Regex::new(r"\b(true|false)\b").unwrap();
    // let string_literal_regex = Regex::new(r#""([^"]*)""#).unwrap();
    let line_arm: Vec<&str> = input.split(" ").collect();
    let mut data_type: Option<String> = None;
    let mut visibility: Option<String> = None;
    let mut name: Option<String> = None;
    let mut value: Option<String> = None;

    let is_custom_data_type = match custum_data_type {
        None => false,
        _ => true,
    };
    // process::exit(1);

    if !DATA_TYPES.contains(&line_arm[0]) && !is_custom_data_type && !line_arm[0].contains(&"[]") {
        if custom_types.contains(&line_arm[0]) {
            return variable_lexing(input, scope, Some(true), custom_types);
        } else {
            print_error(&format!("ERROR: Unidentified data type {}", &line_arm[0]));
            process::exit(1);
        }
    } else {
        if line_arm[0].contains("[]") {
            // println!("{}", &line_arm[0][..line_arm[0].find("[").unwrap()]);
            if !DATA_TYPES.contains(&&line_arm[0][..line_arm[0].find("[").unwrap()])
                && !custom_types.contains(&&line_arm[0][..line_arm[0].find("[").unwrap()])
            {
                print_error(&format!("ERROR: Unidentified data type {}", &line_arm[0]));
            }
        }

        if is_custom_data_type {
            if line_arm.contains(&"=") {
                let mut var_name: Option<String> = None;
                let custom_type = &line_arm[0];
                let use_case = line_arm.join(" ");
                let mut custom_type_arms: Vec<Argument> = Vec::new();

                let id: Vec<&str> = use_case.split("=").collect();
                let left_padding: Vec<&str> =
                    id[0].split(" ").filter(|pred| !pred.is_empty()).collect();
                if left_padding.len() != 3 || !left_padding.contains(&"memory") {
                    if !left_padding.contains(&"memory") {
                        print_error(&format!("Expecting \"memory\" at {}", use_case))
                    } else {
                        print_error(&format!(
                            "Unprocessible entity \"{}\" expecting a variable name",
                            use_case
                        ))
                    }
                } else {
                    var_name = Some(left_padding.last().unwrap().to_string());
                    let right_padding = id[1].trim();

                    if let Some(id_match) = identifier_regex.find(
                        &right_padding[..right_padding
                            .find("(")
                            .expect(&format!("Missing \"(\" at {}", right_padding))],
                    ) {
                        if &id_match.as_str().trim() != custom_type {
                            print_error(&format!(
                                "Invalid value {} for type \"{}\"",
                                id_match.as_str(),
                                custom_type
                            ))
                        } else {
                            let arms = &right_padding[right_padding
                                .find("{")
                                .expect(&format!("Missing {} in \"{}\"", "{", right_padding))
                                + 1
                                ..right_padding
                                    .find("}")
                                    .expect(&format!("Missing {} \"{}\"", "}", right_padding))];

                            for arm in arms.split(",").filter(|pred| !pred.is_empty()) {
                                let arm_expr: Vec<&str> = arm.split(":").collect();
                                if arm_expr.len() != 2 {
                                    print_error(&format!(
                                        "Unprocessible entity at {}",
                                        right_padding
                                    ))
                                } else {
                                    custom_type_arms.push(Argument::Params(
                                        arm_expr[0].to_string(),
                                        arm_expr[1].to_string(),
                                    ));
                                }
                            }
                        }
                    } else {
                        print_error(&format!("Unprocessible entity {}", right_padding))
                    }
                }

                // let struct_variable = Token::StructVariableIdentifier(line_arm[0].to_string(), (), ())
                let token = Token::StructVariableIdentifier(
                    custom_type.to_string(),
                    var_name.unwrap(),
                    custom_type_arms,
                );
                token
            } else {
                let blah = line_arm[1..].join(" ");
                data_type = Some(line_arm[0].to_string());
                if let Some(identifier_match) = &identifier_regex.find(&blah) {
                    if identifier_match.as_str() == "public" {
                        if let Some(blah_match) =
                            identifier_regex.find(&blah[identifier_match.end()..])
                        {
                            name = Some(blah_match.as_str().to_string())
                        }
                    } else if identifier_match.as_str() == "private" {
                        if let Some(blah_match) =
                            identifier_regex.find(&blah[identifier_match.end()..])
                        {
                            name = Some(blah_match.as_str().to_string())
                        }
                    } else {
                        name = Some(identifier_match.as_str().to_string())
                    }
                }

                if let Some(visibility_match) = Regex::new(r"public").unwrap().find(&blah) {
                    visibility = Some(visibility_match.as_str().to_string());
                }

                // println!("{}", blah);
                if blah.contains("=") {
                    value = Some(blah[blah.find("=").unwrap() + 1..].trim().to_string())
                }

                // if let Some(int_match) = int_literal_regex.find(&blah) {
                //     value = Some(int_match.as_str().to_string());
                // }

                // if let Some(string_match) = string_literal_regex.find(&blah) {
                //     value = Some(string_match.as_str().to_string());
                // }

                // if let Some(arr_match) = Regex::new(r"^\[.*\]$").unwrap().find(&blah) {
                //     println!("{}", arr_match.as_str());
                // }

                // if let Some(hex_match) = Regex::new(r"\b(?:0[xX])?[0-9a-fA-F]+\b")
                //     .unwrap()
                //     .find(&blah)
                // {
                //     value = Some(hex_match.as_str().to_string());
                // }

                // if let Some(bool_match) = bool_literal_regex.find(&blah) {
                //     value = Some(bool_match.as_str().to_string());
                // }

                if let None = data_type {
                    print_error("Invalid syntax");
                }

                if let None = visibility {
                    visibility = Some("private".to_string());
                }

                if let None = name {
                    print_error("Variable name required");
                }

                // if let None = value {
                //     print_error("Missing value for variable")
                // }

                let token = Token::VariableIdentifier(
                    data_type.unwrap(),
                    visibility.unwrap(),
                    name.unwrap(),
                    value,
                    scope,
                );
                token
            }
        } else {
            let blah = line_arm[1..].join(" ");
            data_type = Some(line_arm[0].to_string());
            if let Some(identifier_match) = &identifier_regex.find(&blah) {
                if identifier_match.as_str() == "public" {
                    if let Some(blah_match) = identifier_regex.find(&blah[identifier_match.end()..])
                    {
                        name = Some(blah_match.as_str().to_string())
                    }
                } else if identifier_match.as_str() == "private" {
                    if let Some(blah_match) = identifier_regex.find(&blah[identifier_match.end()..])
                    {
                        name = Some(blah_match.as_str().to_string())
                    }
                } else {
                    name = Some(identifier_match.as_str().to_string())
                }
            }

            if let Some(visibility_match) = Regex::new(r"public").unwrap().find(&blah) {
                visibility = Some(visibility_match.as_str().to_string());
            }

            // println!("{}", blah);
            if blah.contains("=") {
                value = Some(blah[blah.find("=").unwrap() + 1..].trim().to_string())
            }

            // if let Some(int_match) = int_literal_regex.find(&blah) {
            //     value = Some(int_match.as_str().to_string());
            // }

            // if let Some(string_match) = string_literal_regex.find(&blah) {
            //     value = Some(string_match.as_str().to_string());
            // }

            // if let Some(arr_match) = Regex::new(r"^\[.*\]$").unwrap().find(&blah) {
            //     println!("{}", arr_match.as_str());
            // }

            // if let Some(hex_match) = Regex::new(r"\b(?:0[xX])?[0-9a-fA-F]+\b")
            //     .unwrap()
            //     .find(&blah)
            // {
            //     value = Some(hex_match.as_str().to_string());
            // }

            // if let Some(bool_match) = bool_literal_regex.find(&blah) {
            //     value = Some(bool_match.as_str().to_string());
            // }

            if let None = data_type {
                print_error("Invalid syntax");
            }

            if let None = visibility {
                visibility = Some("private".to_string());
            }

            if let None = name {
                print_error("Variable name required");
            }

            // if let None = value {
            //     print_error("Missing value for variable")
            // }

            let token = Token::VariableIdentifier(
                data_type.unwrap(),
                visibility.unwrap(),
                name.unwrap(),
                value,
                scope,
            );
            token
        }
    }
}

fn function_lexing(input: Vec<&str>, custom_types: &Vec<&str>) -> Token {
    let identifier_regex = Regex::new(r"[a-zA-Z_]\w*").unwrap();
    let mut func_name: Option<String> = None;
    let mut visibility: Option<String> = None;
    let mut view: Option<String> = None;
    let mut returns: Option<String> = None;
    let mut arms: Vec<Token> = Vec::new();
    let mut gasless: bool = false;
    let mut payable: bool = false;
    let mut raw_args_collection: Vec<Argument> = Vec::new();
    let mut args_collection: Option<Vec<Argument>> = None;

    if let Some(identifier_match) = identifier_regex.find(&input.join(" ")) {
        if identifier_match.as_str() == "function" {
            if let Some(var_name_match) =
                identifier_regex.find(&input.join(" ")[identifier_match.end()..])
            {
                if var_name_match.as_str() == "public"
                    || identifier_match.as_str() == "view"
                    || identifier_match.as_str() == "returns"
                    || identifier_match.as_str() == "gasless"
                    || identifier_match.as_str() == "payable"
                {
                    func_name = None;
                } else {
                    func_name = Some(var_name_match.as_str().to_string());
                    let args = &input.join(" ")[identifier_match.end()..][var_name_match.end()..];
                    if args.starts_with("(") {
                        let closing_tag_index = args.find(")");
                        if let None = closing_tag_index {
                            print_error("Missing argument closing tag");
                        } else {
                            let args_str = &args[1..closing_tag_index.unwrap()];
                            for raw_arg in args_str
                                .split(",")
                                .filter(|predicate| !predicate.is_empty())
                            {
                                let check_arg_len: Vec<&str> = raw_arg
                                    .trim()
                                    .split(" ")
                                    .filter(|pred| !pred.is_empty())
                                    .collect();
                                if check_arg_len.len() != 2 {
                                    print_error(
                                        format!("Invalid function argument \"{}\"", raw_arg)
                                            .as_str(),
                                    )
                                } else {
                                    if !DATA_TYPES.contains(&check_arg_len[0])
                                        && !check_arg_len[0].contains("[]")
                                    {
                                        print_error(&format!(
                                            "Invalid datatype \"{}\"",
                                            check_arg_len[0]
                                        ))
                                    }
                                    let arg_enum = Argument::Params(
                                        check_arg_len[0].to_string(),
                                        check_arg_len[1].to_string(),
                                    );
                                    raw_args_collection.push(arg_enum)
                                }
                            }
                        }
                    } else {
                        print_error(&format!("Expecting \"(\" but found \"{}\"", &args[..1]));
                    }
                }
            }
        }
    }

    if let None = func_name {
        print_error("Function name required")
    }

    if let Some(visibility_match) = Regex::new(r"\b(public|private|external)\b")
        .unwrap()
        .find(&input[0])
    {
        visibility = Some(visibility_match.as_str().trim().to_string())
    } else {
        visibility = Some("private".to_string())
    }

    if let Some(view_match) = Regex::new(r"\b(view|pure)\b").unwrap().find(&input[0]) {
        view = Some(view_match.as_str().trim().to_string())
    }

    if let Some(_) = Regex::new(r"\b(gasless)\b").unwrap().find(&input[0]) {
        gasless = true;
    }

    if let Some(_) = Regex::new(r"\b(payable)\b").unwrap().find(&input[0]) {
        payable = true;
    }

    if let Some(returns_match) = Regex::new(r"returns\(([^)]*)\)").unwrap().find(&input[0]) {
        returns = Some(returns_match.as_str().trim().to_string())
    }

    for expr_arm in input[1..input.len() - 1]
        .join(" ")
        .lines()
        .filter(|ft| !ft.trim().starts_with("//") && !ft.is_empty())
    {
        let check_double: Vec<&str> = expr_arm.trim().split(";").collect();
        for db_lines in check_double {
            if !db_lines.trim().is_empty() {
                if db_lines.trim().starts_with("require") {
                    let use_case = db_lines.trim();
                    let require_keyword_index = use_case.find("(");
                    if let None = require_keyword_index {
                        print_error(&format!("ERROR: Unidentified argument {}", use_case));
                    } else {
                        let striped_expr =
                            &use_case[require_keyword_index.unwrap() + 1..use_case.len() - 1];

                        let require_arm: Vec<&str> = striped_expr.split(",").collect();
                        if require_arm.len() != 2 {
                            print_error(&format!(
                                "expected 2 arguments found {} in \"{}\"",
                                require_arm.len(),
                                db_lines.trim()
                            ));
                        } else {
                            let tokenized_expr: Token = Token::Require(
                                require_arm[0].trim().to_string(),
                                require_arm[1].trim().to_string(),
                            );

                            arms.push(tokenized_expr);
                        }
                    }
                } else {
                    let data_type: Vec<&str> = db_lines
                        .split(" ")
                        .filter(|pred| !pred.is_empty())
                        .collect();
                    if DATA_TYPES.contains(&data_type[0].trim()) {
                        let tokenized_expr: Token = variable_lexing(
                            db_lines.trim(),
                            Scope::Functional(func_name.clone().unwrap()),
                            None,
                            custom_types,
                        );
                        arms.push(tokenized_expr);
                    } else {
                        if data_type.len() != 3 {
                            print_error(&format!(
                                "Unprocessible entity \"{}\"",
                                data_type.join(" ")
                            ))
                        } else {
                            arms.push(Token::Assignment(
                                data_type[0].to_string(),
                                data_type[2].to_string(),
                            ))
                        }
                    }
                }
            }
        }
    }

    if let None = visibility {
        visibility = Some("private".to_string());
    }

    if !raw_args_collection.is_empty() {
        args_collection = Some(raw_args_collection);
    }

    let function_identifier = FunctionIdentifier::new(
        func_name.unwrap(),
        visibility.unwrap(),
        view,
        arms,
        returns,
        gasless,
        payable,
        args_collection,
    );

    Token::FunctionIdentifier(function_identifier)
}

fn print_error(msg: &str) {
    panic!("ERROR: {msg}");
}
