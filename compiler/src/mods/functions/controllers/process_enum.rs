use crate::mods::{
    functions::helpers::helpers::{
        extract_custom_data_types_tokens, print_error, validate_identifier_regex,
    },
    types::types::{EnumIdentifier, LineDescriptions, Token},
};

pub fn extract_enum(data: &Vec<LineDescriptions>) -> Vec<EnumIdentifier> {
    let extracted_enums = extract_custom_data_types_tokens(&Token::Enum, data);
    let mut enum_identifier: Vec<EnumIdentifier> = Vec::new();

    for enum_inst in extracted_enums {
        let mut _identifier: Option<String> = None;
        if let Token::Identifier(_id) = &enum_inst[1] {
            _identifier = Some(_id.to_string());
        } else {
            print_error("Missing enum identifier!!");
        }

        let stripped = &enum_inst[3..enum_inst.len() - 1];
        let splited: Vec<&[Token]> = stripped.split(|pred| pred == &Token::Coma).collect();
        let mut combined_types: Vec<String> = Vec::new();
        for splited_param in splited.iter().filter(|pred| !pred.is_empty()) {
            if !splited_param.is_empty() {
                if splited_param.len() != 1 {
                    print_error(&format!("Invalid enum variant ",))
                }
                if let Token::Identifier(_id) = &splited_param[0] {
                    validate_identifier_regex(_id, 000);
                    combined_types.push(_id.to_string());
                } else {
                    print_error("Invalid enum variant")
                }
            }
        }

        let structured = EnumIdentifier {
            identifier: _identifier.unwrap(),
            variants: combined_types,
        };

        enum_identifier.push(structured);
    }

    enum_identifier
}
