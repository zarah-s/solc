use std::{
    env,
    time::{self, SystemTime},
};
mod mods;

use mods::{
    functions::controllers::{
        process_enum::extract_enum, process_file_contents::process_file_contents,
        process_function::extract_functions, process_state_variables::extract_global_elements,
        process_struct::extract_struct, strip_comments::strip_comments,
        structure_to_line_descriptors::structure_to_line_descriptors,
    },
    types::types::{ContractIdentifier, LineDescriptions, Token},
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

    let mut opened_braces = 0;
    let mut combo: Vec<LineDescriptions> = Vec::new();
    let mut joined: Vec<Vec<LineDescriptions>> = Vec::new();
    for data in structured_stripped_compilable_contents {
        let raw = &data.text;
        combo.push(data.clone());

        if raw.contains("{") {
            for character in raw.chars() {
                if character == '{' {
                    opened_braces += 1;
                }
            }
        }

        if raw.contains("}") {
            for character in raw.chars() {
                if character == '}' {
                    opened_braces -= 1;

                    if opened_braces == 0 {
                        joined.push(combo.clone());
                        combo.clear();
                    }
                }
            }
        }
    }

    let mut contract_construct: Vec<ContractIdentifier> = Vec::new();

    for ddd in joined {
        let extracted_enums = extract_enum(&ddd);

        let structs_tree = extract_struct(&ddd);
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

        let (state_variables, custom_errors, mappings, events) =
            extract_global_elements(&ddd, &custom_data_types_identifiers, &enum_identifiers);

        let (functions, contract_identifier, contract_inheritance) = extract_functions(
            &ddd,
            &custom_data_types_identifiers,
            &state_variables,
            &enum_identifiers,
            &mappings,
        );
        let contract_identifier = ContractIdentifier {
            identifier: contract_identifier,
            inheritance: if contract_inheritance.is_empty() {
                None
            } else {
                Some(contract_inheritance)
            },
            custom_errors,
            enums: extracted_enums,
            events,
            functions,
            mappings,
            state_variables,
            structs: structs_tree,
        };
        contract_construct.push(contract_identifier)
    }

    println!("{:#?}", contract_construct);

    // println!(
    //     "===> STRUCT ===>\n{:#?}\n\n ===> GLOBAL_VARIABLES ===>\n{:#?}\n\n ===> MAPPINGS ===>\n{:#?}\n\n ===> ENUMS ===>\n{:#?}\n\n ===>> CUSTOM_ERRORS ==>>\n{:#?}\n\n ===>> FUNCTIONS ==>>\n{:#?}",
    //     structs_tree, global_variables,mappings, extracted_enums, custom_errors,functions
    // );

    let end_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!(
        "Program completed in \x1b[93m{:?}\x1b[0m",
        (end_time.unwrap() - start_time.unwrap())
    );

    Ok(())
}
