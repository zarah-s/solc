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
            process_file_contents, strip_comments,
            structure_to_line_descriptors::{self},
        },
        types::types::LineDescriptions,
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
            println!("{:?} stripped", stripped_comments);
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
        use crate::mods::functions::controllers::process_state_variables::extract_global_variables;

        use super::get_file_contents;

        #[tokio::test]
        async fn test_variable_count() {
            let contents = get_file_contents("test/files/vars/Var.sol").await;
            let (_vars, _, _) = extract_global_variables(&contents, &Vec::new(), &Vec::new());
            assert_eq!(_vars.len(), 2);
        }

        #[tokio::test]
        #[should_panic(
            expected = "ERROR: Invalid data type \"strings public text = \"Hello\"\" on line  6"
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
        #[should_panic(expected = "ERROR: Missing \"]\" on line 9")]
        async fn test_invalid_syntax_close_bracket_for_arr_vars() {
            let contents = get_file_contents("test/files/vars/Var4.sol").await;
            extract_global_variables(&contents, &Vec::new(), &Vec::new());
        }
    }

    //******************************** HELPER FUNCTIONS***************** */
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
