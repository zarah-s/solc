use std::{
    env,
    time::{self, SystemTime},
};
mod mods;

use mods::{
    functions::controllers::{
        process_enum::extract_enum, process_file_contents::process_file_contents,
        process_function::extract_functions, process_state_variables::extract_global_variables,
        process_struct::extract_struct, strip_comments::strip_comments,
        structure_to_line_descriptors::structure_to_line_descriptors,
    },
    types::types::{LineDescriptions, Token},
};
use tokio::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let start_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    /* GET ENVIRONMENT ARGUMENTS */
    let args: Vec<String> = env::args().collect();

    /* LINES DESCRIPTION CONTAINING LINE NUMBER */
    let mut lines_: Vec<LineDescriptions> = Vec::new();
    let mut stripped_comments = String::new();
    let mut file_contents = String::new();
    let _ = process_file_contents(args, &mut file_contents).await?;

    structure_to_line_descriptors(&file_contents, &mut lines_);

    strip_comments(&lines_, &mut stripped_comments);
    let structured_stripped_compilable_contents: Vec<LineDescriptions> =
        LineDescriptions::to_struct(stripped_comments);
    let extracted_enums = extract_enum(&structured_stripped_compilable_contents);

    let structs_tree = extract_struct(&structured_stripped_compilable_contents);
    let struct_identifiers: Vec<&str> = structs_tree
        .iter()
        .map(|pred| pred.identifier.as_str())
        .collect();

    let enum_identifiers: Vec<&str> = extracted_enums
        .iter()
        .map(|pred| pred.identifier.as_str())
        .collect();

    let custom_data_types_identifiers: Vec<&str> =
        [enum_identifiers.clone(), struct_identifiers].concat();

    let (global_variables, custom_errors, mappings) = extract_global_variables(
        &structured_stripped_compilable_contents,
        &custom_data_types_identifiers,
        &enum_identifiers,
    );

    let functions = extract_functions(
        &structured_stripped_compilable_contents,
        &custom_data_types_identifiers,
        &global_variables,
        &enum_identifiers,
        &mappings,
    );

    println!(
        "===> STRUCT ===>\n{:#?}\n\n ===> GLOBAL_VARIABLES ===>\n{:#?}\n\n ===> MAPPINGS ===>\n{:#?}\n\n ===> ENUMS ===>\n{:#?}\n\n ===>> CUSTOM_ERRORS ==>>\n{:#?}\n\n ===>> FUNCTIONS ==>>\n{:#?}",
        structs_tree, global_variables,mappings, extracted_enums, custom_errors,functions
    );

    let end_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!(
        "Program completed in \x1b[93m{:?}\x1b[0m",
        (end_time.unwrap() - start_time.unwrap())
    );

    Ok(())
}
