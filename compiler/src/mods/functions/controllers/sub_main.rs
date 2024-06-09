use std::{collections::HashMap, path::Path};

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
    import_tree: &mut HashMap<String, Vec<String>>,
    compiled: &mut [Vec<String>; 4],
) -> Result<(), std::io::Error> {
    if args.len() < 2 {
        print_error("Mising file path... Run cargo run <file-path>")
    }

    /* VALIDATE FILE FORMAT */
    if args[1].split(".").last().unwrap() != "sol" {
        print_error("Unsupported file... Expected \".sol\" file");
    }
    let file_path = &args[1];
    let root_folder = Path::new(&file_path);
    let parent_folder = Path::new(&args[0]);

    {
        for key in import_tree.keys() {
            let entries = import_tree.get(key);
            if let Some(_entries) = entries {
                let mut current_index = 0;
                for (index, __entry) in _entries.iter().enumerate() {
                    if current_index != index {
                        if *__entry == _entries[current_index] {
                            print_error("Recursive imports")
                        }
                    }
                }
                current_index += 1;
                if current_index != 0 {}
            }
        }
    }

    let root = &root_folder
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let parent_root = &parent_folder
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    {
        let find = import_tree.get(parent_root);
        let mut data = Vec::new();

        if find.is_some() {
            for i in find.unwrap() {
                data.push(i.clone())
            }
        }
        data.push(root.to_string());

        import_tree.insert(parent_root.to_string(), data);
    }

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
                vec![root.to_string(), joined.to_str().unwrap().to_string()],
                abstract_contracts,
                main_contracts,
                libraries,
                interfaces,
                import_tree,
                compiled,
            )
            .await;
        })
        .await;
    }

    for _joined in joined {
        {
            let mut should_compile = true;
            for __joined in &_joined {
                if __joined.text.starts_with("contract")
                    || __joined.text.starts_with("library")
                    || __joined.text.starts_with("interface")
                {
                    let splitted = __joined.text.split(" ").collect::<Vec<_>>();
                    if __joined.text.starts_with("contract") {
                        if compiled[0].contains(&splitted[1].to_string()) {
                            should_compile = false
                        }
                    } else if __joined.text.starts_with("interface") {
                        if compiled[1].contains(&splitted[1].to_string()) {
                            should_compile = false
                        }
                    } else if __joined.text.starts_with("library") {
                        if compiled[2].contains(&splitted[1].to_string()) {
                            should_compile = false
                        }
                    }
                } else if __joined.text.starts_with("abstract") {
                    let splitted = __joined.text.split(" ").collect::<Vec<_>>();
                    if compiled[3].contains(&splitted[1].to_string()) {
                        should_compile = false
                    }
                }
            }

            if !should_compile {
                continue;
            }
        }
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
            &libraries,
        );
        let (functions, contract_header, _libraries) = extract_functions(
            &_joined,
            &custom_data_types_identifiers,
            &state_variables,
            &enum_identifiers,
            &mappings,
            interfaces,
            &mut compiled[1],
            &libraries,
        );

        for _library in _libraries {
            compiled[2].push(_library.identifier.to_string());
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

            compiled[3].push(contract_identifier.header.identifier.to_string());
            if let ContractType::Main = contract_identifier.header.r#type {
                main_contracts.push(contract_identifier)
            } else if let ContractType::Abstract = contract_identifier.header.r#type {
                abstract_contracts.push(contract_identifier)
            }
        }
    }

    Ok(())
}
