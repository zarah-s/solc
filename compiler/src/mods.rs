pub mod types {
    pub mod types;
}

pub mod constants {
    pub mod constants;
}

pub mod functions {
    pub mod helpers {
        pub mod helpers;
    }

    pub mod controllers {
        pub mod process_enum;
        pub mod process_file_contents;
        pub mod process_function;
        pub mod process_state_variables;
        pub mod process_struct;
        pub mod strip_comments;
        pub mod structure_to_line_descriptors;
    }
}

pub mod implementations {
    pub mod conditionals;
    pub mod line_descriptors;
    pub mod mapping;
}

#[cfg(test)]

mod tests {
    use super::{
        functions::controllers::{
            process_enum::extract_enum, process_file_contents, process_function::extract_functions,
            process_state_variables::extract_global_variables, process_struct::extract_struct,
            strip_comments, structure_to_line_descriptors,
        },
        types::types::{FunctionsIdentifier, LineDescriptions},
    };

    mod file_processing {
        use crate::mods::{
            functions::controllers::{
                process_file_contents,
                strip_comments::{self},
                structure_to_line_descriptors,
            },
            types::types::LineDescriptions,
        };

        #[tokio::test]
        #[should_panic(expected = "ERROR: Mising file path... Run cargo run <file-path>")]
        async fn test_empty_args() {
            let args: Vec<String> = vec![];
            let mut file_contents = String::new();
            let _ = process_file_contents::process_file_contents(args, &mut file_contents).await;
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Unsupported file... Expected \".sol\" file")]
        async fn test_valid_file_format() {
            let args: Vec<String> =
                vec![String::from("run executable"), String::from("Contract.txt")];
            let mut file_contents = String::new();
            let _ = process_file_contents::process_file_contents(args, &mut file_contents).await;
        }

        #[test]
        fn test_strip_comments_and_doc_strings() {
            let file_contents = String::from(
                "\n// hello world\n
/* This is a test file or so */\n\nHello world",
            );

            let mut lines_: Vec<LineDescriptions> = Vec::new();
            let mut stripped_comments = String::new();

            structure_to_line_descriptors::structure_to_line_descriptors(
                &file_contents,
                &mut lines_,
            );
            strip_comments::strip_comments(&lines_, &mut stripped_comments);
            assert!(!stripped_comments.contains("//"));
            assert!(!stripped_comments.contains("/*"));
            assert!(!stripped_comments.contains("*/"));
        }
    }

    mod enum_processing {
        use crate::mods::{
            functions::{
                controllers::process_enum::extract_enum,
                helpers::helpers::extract_custom_data_types_tokens,
            },
            types::types::Token,
        };

        use super::get_file_contents;

        #[tokio::test]
        async fn test_enums_count() {
            let contents = get_file_contents("test/files/enums/Enum.sol").await;
            let extracted_enums = extract_custom_data_types_tokens(&Token::Enum, &contents);
            assert_eq!(extracted_enums.len(), 1)
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Missing enum identifier!!")]
        async fn test_enum_identifier() {
            let contents = get_file_contents("test/files/enums/Enum2.sol").await;
            extract_enum(&contents);
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Invalid enum variant")]
        async fn test_variants_integrity() {
            let contents = get_file_contents("test/files/enums/Enum3.sol").await;
            extract_enum(&contents);
        }

        #[tokio::test]
        async fn test_variants_integrity_verbosity() {
            let contents = get_file_contents("test/files/enums/Enum.sol").await;
            let enums: Vec<crate::mods::types::types::EnumIdentifier> = extract_enum(&contents);
            assert_eq!(enums.len(), 1);
            assert_eq!(enums[0].identifier, String::from("Status"));
            assert_eq!(enums[0].variants.len(), 5);
            let variants = vec!["Pending", "Shipped", "Accepted", "Rejected", "Canceled"];
            for (index, ens) in enums[0].variants.iter().enumerate() {
                assert_eq!(ens, &variants[index].to_string())
            }
        }
    }

    mod struct_processing {
        use crate::mods::{
            functions::{
                controllers::process_struct::extract_struct,
                helpers::helpers::extract_custom_data_types_tokens,
            },
            types::types::Token,
        };

        use super::get_file_contents;

        #[tokio::test]
        async fn test_struct_len() {
            let contents = get_file_contents("test/files/struct/Struct.sol").await;
            let structs = extract_custom_data_types_tokens(&Token::Struct, &contents);
            assert_eq!(structs.len(), 2);
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Missing struct identifier!!")]
        async fn test_struct_identifier() {
            let contents = get_file_contents("test/files/struct/Struct2.sol").await;
            extract_struct(&contents);
        }

        #[tokio::test]
        async fn test_struct_with_dynamic_arr() {
            let contents = get_file_contents("test/files/struct/Struct4.sol").await;
            let strs = extract_struct(&contents);
            assert_eq!(strs[0].types[0].is_array, true);
            assert_eq!(strs[0].types[0].size, None);
        }

        #[tokio::test]
        async fn test_struct_with_fixed_arr() {
            let contents = get_file_contents("test/files/struct/Struct4.sol").await;
            let strs = extract_struct(&contents);
            assert_eq!(strs[1].types[0].is_array, true);
            assert_eq!(strs[1].types[0].size, Some("(10*5)/num".to_string()));
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Invalid array size 0")]
        async fn test_struct_with_fixed_arr_panic_if_size_is_zero() {
            let contents = get_file_contents("test/files/struct/Struct6.sol").await;
            extract_struct(&contents);
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Invalid Struct variants")]
        async fn test_variants_integrity() {
            let contents = get_file_contents("test/files/struct/Struct3.sol").await;
            extract_struct(&contents);
        }

        #[tokio::test]
        async fn test_variants_integrity_verbosity() {
            let contents = get_file_contents("test/files/struct/Struct.sol").await;
            let struct_: Vec<crate::mods::types::types::StructIdentifier> =
                extract_struct(&contents);
            assert_eq!(struct_.len(), 2);
            assert_eq!(struct_[0].identifier, String::from("Todo"));
            assert_eq!(struct_[0].types.len(), 2);
            let variants = vec!["text", "completed"];
            for (index, ens) in struct_[0].types.iter().enumerate() {
                assert_eq!(ens.name_, variants[index].to_string())
            }
        }
    }

    mod global_variables_processing {
        use crate::mods::{
            functions::controllers::process_state_variables::extract_global_variables,
            types::types::{Mapping, MappingIdentifier, Token, VariableIdentifier, VariableType},
        };

        use super::get_file_contents;

        #[tokio::test]
        async fn test_variable_count() {
            let contents = get_file_contents("test/files/vars/Var.sol").await;
            let (_vars, _, _) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            assert_eq!(_vars.len(), 2);
        }

        #[tokio::test]
        async fn test_custom_error_count() {
            let contents = get_file_contents("test/files/vars/Error.sol").await;
            let (_, _errs, _) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            assert_eq!(_errs.len(), 1);
        }

        #[tokio::test]
        async fn test_custom_error_intergrity() {
            let contents = get_file_contents("test/files/vars/Error.sol").await;
            let (_, _errs, _) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            assert_eq!(
                _errs[0],
                "error InsufficientBalance(uint256 balance, uint256 withdrawAmount)"
            );
        }

        #[tokio::test]
        async fn test_mapping_count() {
            let contents = get_file_contents("test/files/vars/Map.sol").await;
            let (_, _, _maps) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            assert_eq!(_maps.len(), 2);
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Unprocessible entity on mapping")]
        async fn test_mapping_identifier() {
            let contents = get_file_contents("test/files/vars/Map1.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Mapping can not be set to external")]
        async fn test_mapping_external_visibility() {
            let contents = get_file_contents("test/files/vars/Map2.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Unprocessible entity on mapping")]
        async fn test_mapping_closing_parenthesis() {
            let contents = get_file_contents("test/files/vars/Map3.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Invalid data type \"addresss\"")]
        async fn test_mapping_data_type() {
            let contents = get_file_contents("test/files/vars/Map4.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        async fn test_mapping_integrity() {
            let contents = get_file_contents("test/files/vars/Map.sol").await;
            let expected = vec![
                MappingIdentifier {
                    name: "myMap".to_string(),
                    visibility: Token::Public,
                    map: Mapping {
                        key: Some("address".to_string()),
                        value: Some(crate::mods::types::types::MappingValue::Raw(
                            "uint256".to_string(),
                        )),
                    },
                },
                MappingIdentifier {
                    name: "nested".to_string(),
                    visibility: Token::Public,
                    map: Mapping {
                        key: Some("address".to_string()),
                        value: Some(crate::mods::types::types::MappingValue::Mapping(Box::new(
                            Mapping {
                                key: Some("uint256".to_string()),
                                value: Some(crate::mods::types::types::MappingValue::Raw(
                                    "bool".to_string(),
                                )),
                            },
                        ))),
                    },
                },
            ];
            let (_, _, _maps) = extract_global_variables(&contents, &Vec::new(), &Vec::new());

            for (i, _map) in _maps.iter().enumerate() {
                let val = &expected[i];
                assert_eq!(_map, val);
            }
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Invalid data type \"strings public text = \"Hello\"\" on line 6"
        )]
        async fn test_variable_data_type() {
            let contents = get_file_contents("test/files/vars/Var2.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Missing \"]\" on line 8")]
        async fn test_variable_missing_close_bracket_for_arr_vars() {
            let contents = get_file_contents("test/files/vars/Var3.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        async fn test_variable_dynamic_arr() {
            let contents = get_file_contents("test/files/vars/Var6.sol").await;
            let (_vars, _, _) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            assert_eq!(_vars[0].is_array, true);
            assert_eq!(_vars[0].size, None);
        }

        #[tokio::test]
        async fn test_variable_fixed_arr() {
            let contents = get_file_contents("test/files/vars/Var6.sol").await;
            let (_vars, _, _) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            assert_eq!(_vars[1].is_array, true);
            assert_eq!(_vars[1].size, Some("10*10".to_string()));
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Missing \"]\" on line 8")]
        async fn test_invalid_syntax_close_bracket_for_arr_vars() {
            let contents = get_file_contents("test/files/vars/Var4.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        #[should_panic(expected = "Unprocessible entity bool = false")]
        async fn test_var_identifier() {
            let contents = get_file_contents("test/files/vars/Var5.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }

        #[tokio::test]
        async fn test_variable_integrity() {
            let contents = get_file_contents("test/files/vars/Var.sol").await;
            let (vars, _, _) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            let expected_vars = vec![
                VariableIdentifier {
                    data_type: Token::String,
                    mutability: Token::Mutable,
                    visibility: Token::Public,
                    is_array: false,
                    name: "text".to_string(),
                    size: None,
                    value: Some("\"Hello\"".to_string()),
                    storage_location: None,
                    type_: VariableType::Variable,
                },
                VariableIdentifier {
                    data_type: Token::Uint256,
                    mutability: Token::Mutable,
                    visibility: Token::Public,
                    is_array: false,
                    name: "num".to_string(),
                    size: None,
                    value: Some("123".to_string()),
                    storage_location: None,
                    type_: VariableType::Variable,
                },
            ];
            assert_eq!(vars.len(), 2);

            for (i, var) in vars.iter().enumerate() {
                assert_eq!(var.is_array, expected_vars[i].is_array);
                assert_eq!(var.data_type, expected_vars[i].data_type);
                assert_eq!(var.type_, expected_vars[i].type_);
                assert_eq!(var.visibility, expected_vars[i].visibility);
                assert_eq!(var.mutability, expected_vars[i].mutability);
                assert_eq!(var.name, expected_vars[i].name);
                assert_eq!(var.value, expected_vars[i].value);
                assert_eq!(var.size, expected_vars[i].size);
                assert_eq!(var.storage_location, expected_vars[i].storage_location);
            }
        }
    }

    mod function_processing {
        use super::{get_file_contents, get_fns};
        use crate::mods::{
            functions::controllers::process_function::extract_functions,
            types::types::{
                FunctionArm, FunctionMutability, FunctionsIdentifier, MappingAssign, Token,
                VariableAssign, VariableAssignOperation, VariableAssignType, VariableIdentifier,
                VariableType,
            },
        };

        #[tokio::test]
        async fn test_function_count() {
            let contents = get_file_contents("test/files/function/Fn.sol").await;
            let functions = extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );

            assert_eq!(functions.len(), 4)
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Unprocessible function name \"(uint256\"")]
        async fn test_function_name() {
            let contents = get_file_contents("test/files/function/Fn2.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Invalid Identifier \"@testRequire\" on line 0")]
        async fn test_function_name_validity() {
            let contents = get_file_contents("test/files/function/Fn3.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Cannot declare \"memory\" or \"calldata\" to a primitive type"
        )]
        async fn test_function_arg_when_memo_is_assigned_to_primitive_data_types() {
            let contents = get_file_contents("test/files/function/Fn6.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        async fn test_function_name_match() {
            let contents = get_file_contents("test/files/function/Fn.sol").await;
            let fn_names = vec!["testRequire", "testRevert", "testAssert", "testCustomError"];
            let fns = extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );

            for (i, _fn) in fns.iter().enumerate() {
                match _fn {
                    FunctionsIdentifier::FunctionIdentifier(__fn) => {
                        assert_eq!(__fn.name, fn_names[i]);
                    }
                    _ => (),
                }
            }
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Invalid function argument function testRequire ( _i ) public pure"
        )]
        async fn test_function_arg_revert_less_arg_lexems() {
            let contents = get_file_contents("test/files/function/Fn8.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Unprocessible entity \"uint257\"")]
        async fn test_function_arg_data_type() {
            let contents = get_file_contents("test/files/function/Fn9.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Syntax error... Expecting a close bracket for function testRequire ( uint256 [ memory _i ) public pure"
        )]
        async fn test_function_arg_close_brack_for_arr_values() {
            let contents = get_file_contents("test/files/function/Fn10.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Invalid function argument. Payable for a non address type function testRequire ( uint256 payable _i ) public pure"
        )]
        async fn test_function_arg_payable_annotation_to_non_address_type() {
            let contents = get_file_contents("test/files/function/Fn11.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Expecting \"memory\" or \"calldata\". function testRequire ( string _i ) public pure "
        )]
        async fn test_function_arg_panic_if_non_primitive_type_is_not_assigned_memory() {
            let contents = get_file_contents("test/files/function/Fn12.sol").await;
            extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );
        }

        #[tokio::test]
        async fn test_function_args_integrity() {
            let contents = get_file_contents("test/files/function/Fn7.sol").await;
            let functions = extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );

            match &functions[0] {
                FunctionsIdentifier::FunctionIdentifier(_function) => {
                    assert_eq!(_function.arguments.len(), 1);
                    assert_eq!(_function.arguments[0].is_array, false);
                    assert_eq!(_function.arguments[0].location, None);
                    assert_eq!(_function.arguments[0].name_, "_i".to_string());
                    assert_eq!(_function.arguments[0].payable_address, false);
                    assert_eq!(_function.arguments[0].size, None);
                    assert_eq!(_function.arguments[0].type_, "uint256".to_string());
                }
                _ => (),
            }
        }

        #[tokio::test]
        async fn test_function_visibility() {
            let contents = get_file_contents("test/files/function/Fn4.sol").await;
            let fn_visibilities = vec![
                Token::Public,
                Token::Internal,
                Token::External,
                Token::Private,
            ];
            let fns = extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );

            for (i, _fn) in fns.iter().enumerate() {
                match _fn {
                    FunctionsIdentifier::FunctionIdentifier(__fn) => {
                        assert_eq!(__fn.visibility, fn_visibilities[i]);
                    }
                    _ => (),
                }
            }
        }

        #[tokio::test]
        async fn test_function_mutability() {
            let contents = get_file_contents("test/files/function/Fn5.sol").await;
            let fn_mutabilities = vec![
                FunctionMutability::View,
                FunctionMutability::Pure,
                FunctionMutability::Mutable,
                FunctionMutability::Mutable,
            ];
            let fns = extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );

            for (i, _fn) in fns.iter().enumerate() {
                match _fn {
                    FunctionsIdentifier::FunctionIdentifier(__fn) => {
                        assert_eq!(__fn.mutability, fn_mutabilities[i]);
                    }
                    _ => (),
                }
            }
        }

        #[tokio::test]
        async fn test_function_returns_integrity() {
            let contents = get_file_contents("test/files/function/Fn13.sol").await;

            let fns = extract_functions(
                &contents,
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
                &Vec::new(),
            );

            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    let __d = _fn.returns.as_ref().unwrap();
                    assert_eq!(__d.len(), 3);
                    assert_eq!(__d[0].type_, "uint");
                    assert_eq!(__d[0].is_array, false);
                    assert_eq!(__d[0].location, None);
                    assert_eq!(__d[1].type_, "string");
                    assert_eq!(__d[1].is_array, false);
                    assert_eq!(__d[1].location, Some(Token::Memory));
                    assert_eq!(__d[2].location, Some(Token::Memory));
                    assert_eq!(__d[2].is_array, true);
                }
                _ => (),
            }
        }

        #[tokio::test]
        async fn test_fn_arm_variable_identifier() {
            let fns = get_fns("test/files/function/Fn14.sol").await;

            let expected = FunctionArm::VariableIdentifier(VariableIdentifier {
                data_type: Token::Identifier("Status".to_string()),
                mutability: Token::Mutable,
                visibility: Token::Internal,
                is_array: false,
                name: "__status".to_string(),
                size: None,
                value: Some("Status.Start".to_string()),
                storage_location: None,
                type_: VariableType::Enum,
            });

            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    assert_eq!(_fn.arms[0], expected)
                }
                _ => (),
            }
        }

        #[tokio::test]
        async fn test_fn_arm_variable_assign() {
            let fns = get_fns("test/files/function/Fn15.sol").await;
            let expected = FunctionArm::VariableAssign(VariableAssign {
                identifier: "__status".to_string(),
                value: "Status.Start".to_string(),
                operation: VariableAssignOperation::Assign,
                type_: VariableAssignType::Enum,
                variant: None,
            });
            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    assert_eq!(_fn.arms[0], expected);
                }
                _ => (),
            }
        }

        #[tokio::test]
        async fn test_fn_arm_variable_push() {
            let fns = get_fns("test/files/function/Fn16.sol").await;
            let expected = FunctionArm::VariableAssign(VariableAssign {
                identifier: "__arr".to_string(),
                value: "2".to_string(),
                operation: VariableAssignOperation::Push,
                type_: VariableAssignType::Array(None),
                variant: None,
            });

            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    assert_eq!(_fn.arms[4], expected);
                }
                _ => (),
            }
        }

        #[tokio::test]
        async fn test_fn_arm_variable_pop() {
            let fns = get_fns("test/files/function/Fn16.sol").await;
            let expected = FunctionArm::VariableAssign(VariableAssign {
                identifier: "__arr".to_string(),
                value: "".to_string(),
                operation: VariableAssignOperation::Pop,
                type_: VariableAssignType::Array(None),
                variant: None,
            });
            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    assert_eq!(_fn.arms[5], expected);
                }
                _ => (),
            }
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Pop method cannot be assigned value")]
        async fn test_fn_arm_panic_variable_pop_if_val_is_passed() {
            get_fns("test/files/function/Fn18.sol").await;
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Cannot call a method on a fixed size array \"__arr\"")]
        async fn test_fn_arm_panic_variable_if_method_is_called_on_fixed_array_var() {
            get_fns("test/files/function/Fn19.sol").await;
        }

        #[tokio::test]
        async fn test_fn_arm_mapping_assign() {
            let fns = get_fns("test/files/function/Fn16.sol").await;
            let expected = FunctionArm::MappingAssign(MappingAssign {
                identifier: "name".to_string(),
                value: "2".to_string(),
                operation: VariableAssignOperation::Assign,
                type_: VariableAssignType::Mapping,
                variants: vec!["msg.sender".to_string()],
            });

            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    assert_eq!(_fn.arms[0], expected);
                }
                _ => (),
            }
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Missing key for mapping assignment. \"names\"")]
        async fn test_fn_arm_panic_mapping_assignment_missing_key() {
            get_fns("test/files/function/Fn22.sol").await;
        }

        #[tokio::test]
        async fn test_fn_arm_mapping_push() {
            let fns = get_fns("test/files/function/Fn16.sol").await;

            let expected = FunctionArm::MappingAssign(MappingAssign {
                identifier: "names".to_string(),
                value: "3".to_string(),
                operation: VariableAssignOperation::Push,
                type_: VariableAssignType::Mapping,
                variants: vec!["msg.sender".to_string()],
            });

            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    assert_eq!(_fn.arms[1], expected);
                }
                _ => (),
            }
        }

        #[tokio::test]
        async fn test_fn_arm_mapping_pop() {
            let fns = get_fns("test/files/function/Fn16.sol").await;

            let expected = FunctionArm::MappingAssign(MappingAssign {
                identifier: "names".to_string(),
                value: "".to_string(),
                operation: VariableAssignOperation::Pop,
                type_: VariableAssignType::Mapping,
                variants: vec!["msg.sender".to_string()],
            });

            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => {
                    assert_eq!(_fn.arms[2], expected);
                }
                _ => (),
            }
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Pop method cannot be assigned value")]
        async fn test_fn_arm_panic_mapping_pop_if_val_is_passed() {
            get_fns("test/files/function/Fn17.sol").await;
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Cannot call a method on a fixed size array \"names\"")]
        async fn test_fn_arm_panic_mapping_if_method_is_called_on_fixed_array_var() {
            get_fns("test/files/function/Fn20.sol").await;
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Invalid data type \"addressd\"")]
        async fn test_fn_arm_panic_mapping_invalid_key_type() {
            get_fns("test/files/function/Fn21.sol").await;
        }

        #[tokio::test]
        async fn test_fn_arm_mapping_variants_integrity() {
            let expected = vec!["address(0)", "0"];
            let fns = get_fns("test/files/function/Fn16.sol").await;
            match &fns[0] {
                FunctionsIdentifier::FunctionIdentifier(_fn) => match _fn.arms[3] {
                    FunctionArm::MappingAssign(ref _d) => {
                        assert_eq!(_d.variants, expected)
                    }
                    _ => (),
                },
                _ => (),
            }
            // assert_eq!(fns[0], expected);
        }

        #[tokio::test]
        async fn test_constructor() {
            let fns = get_fns("test/files/function/Fn23.sol").await;
            match &fns[0] {
                FunctionsIdentifier::ConstructorIdentifier(_fn) => {
                    assert_eq!(_fn.arguments.len(), 1);
                    assert_eq!(_fn.arms.len(), 2);
                }
                _ => (),
            }
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Unprocessible entity for receive function. \"function does not support argument\""
        )]
        async fn test_fn_arm_panic_if_args_is_passed_to_receive_function() {
            get_fns("test/files/function/Fn24.sol").await;
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Expecting \"external\" for receive function")]
        async fn test_fn_arm_panic_if_no_external_visibility_annotation_for_receive_function() {
            get_fns("test/files/function/Fn25.sol").await;
        }

        #[tokio::test]
        #[should_panic(expected = "ERROR: Expecting \"payable\" for receive function")]
        async fn test_fn_arm_panic_if_no_payable_annotation_for_receive_function() {
            get_fns("test/files/function/Fn26.sol").await;
        }
    }

    //******************************** HELPER FUNCTIONS***************** */
    async fn get_fns(path: &str) -> Vec<FunctionsIdentifier> {
        let contents = get_file_contents(path).await;
        let structs_tree = extract_struct(&contents);
        let struct_identifiers: Vec<&str> = structs_tree
            .iter()
            .map(|pred| pred.identifier.as_str())
            .collect();
        let extracted_enums = extract_enum(&contents);

        let enum_identifiers: Vec<&str> = extracted_enums
            .iter()
            .map(|pred| pred.identifier.as_str())
            .collect();
        let custom_data_types_identifiers: Vec<&str> =
            [enum_identifiers.clone(), struct_identifiers].concat();
        let (_vars, _, _maps) =
            extract_global_variables(&contents, &custom_data_types_identifiers, &enum_identifiers);

        let fns = extract_functions(
            &contents,
            &custom_data_types_identifiers,
            &_vars,
            &enum_identifiers,
            &_maps,
        );
        fns
    }

    async fn get_file_contents(path: &str) -> Vec<LineDescriptions> {
        let mut file_contents = String::new();
        let _ = process_file_contents::process_file_contents(
            vec![String::new(), String::from(path)],
            &mut file_contents,
        )
        .await;

        let mut lines_: Vec<LineDescriptions> = Vec::new();
        let mut stripped_comments = String::new();

        structure_to_line_descriptors::structure_to_line_descriptors(&file_contents, &mut lines_);
        strip_comments::strip_comments(&lines_, &mut stripped_comments);

        let structured_stripped_compilable_contents: Vec<LineDescriptions> =
            LineDescriptions::to_struct(stripped_comments);
        structured_stripped_compilable_contents
    }
}
