use std::path::Path;

use crate::mods::{
    functions::helpers::helpers::print_error,
    types::types::{
        ContractIdentifier, ContractType, InterfaceIdentifier, LibraryIdentifier, LineDescriptions,
    },
};

use super::{
    process_enum::extract_enum, process_file_contents::process_file_contents,
    process_function::extract_functions, process_state_variables::extract_global_elements,
    process_struct::extract_struct, strip_comments::strip_comments,
    structure_to_line_descriptors::structure_to_line_descriptors,
};

pub async fn compile_source_code(
    args: Vec<String>,
    abstract_contracts: &mut Vec<ContractIdentifier>,
    main_contracts: &mut Vec<ContractIdentifier>,
    libraries: &mut Vec<LibraryIdentifier>,
    interfaces: &mut Vec<InterfaceIdentifier>,
    compiled_files: &mut Vec<String>,
) -> Result<(), std::io::Error> {
    let file_path = &args[1];
    let root_folder = Path::new(&file_path);
    if compiled_files.contains(
        &root_folder
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    ) {
        print_error(&format!(
            "{:?} has already been imported",
            root_folder.file_name().unwrap()
        ))
    }
    compiled_files.push(
        root_folder
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );
    let mut lines_: Vec<LineDescriptions> = Vec::new();
    let mut stripped_comments = String::new();
    let mut file_contents = String::new();
    let _result = process_file_contents(args.clone(), &mut file_contents).await;
    match _result {
        Ok(_) => (),
        Err(err) => panic!("{} {}", err, file_path),
    }
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

    let mut imports: Vec<String> = Vec::new();

    for __import in &joined[0] {
        if __import.text.starts_with("contract")
            || __import.text.starts_with("abstract")
            || __import.text.starts_with("library")
            || __import.text.starts_with("interface")
        {
            break;
        }

        if __import.text.starts_with("import") {
            let splitted = __import.text.split(" ").collect::<Vec<&str>>();
            if splitted.len() != 2 {
                print_error("Invalid import")
            }

            if splitted[1].is_empty() {
                print_error("Invalid import")
            }

            imports.push(splitted[1][1..splitted[1].len() - 2].to_string())
        }
    }

    for __import in imports {
        let joined = root_folder.parent().unwrap().join(&__import);
        let _ = Box::pin(async {
            let _ = compile_source_code(
                vec![String::new(), joined.to_str().unwrap().to_string()],
                abstract_contracts,
                main_contracts,
                libraries,
                interfaces,
                compiled_files,
            )
            .await;
        })
        .await;
    }

    for _joined in joined {
        let extracted_enums = extract_enum(&_joined);

        let structs_tree = extract_struct(&_joined);
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

        let (state_variables, custom_errors, mappings, events) = extract_global_elements(
            &_joined,
            &custom_data_types_identifiers,
            &enum_identifiers,
            Vec::new(),
        );
        let (functions, contract_header, _libraries) = extract_functions(
            &_joined,
            &custom_data_types_identifiers,
            &state_variables,
            &enum_identifiers,
            &mappings,
            interfaces,
        );
        for _library in _libraries {
            libraries.push(_library);
        }
        if !contract_header.identifier.trim().is_empty() {
            let contract_identifier = ContractIdentifier {
                header: contract_header,
                custom_errors,
                enums: extracted_enums,
                events,
                functions,
                mappings,
                state_variables,
                structs: structs_tree,
            };

            if let ContractType::Main = contract_identifier.header.r#type {
                main_contracts.push(contract_identifier)
            } else if let ContractType::Abstract = contract_identifier.header.r#type {
                abstract_contracts.push(contract_identifier)
            }
        }
    }

    Ok(())
}
