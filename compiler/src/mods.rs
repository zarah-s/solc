pub mod types {
    pub mod compiler_errors;
    pub mod line_descriptors;
    pub mod token;
}

pub mod constants {
    pub mod constants;
}

pub mod functions {
    pub mod helpers {
        // pub mod error_helper;
        // pub mod global;
        // pub mod token_helper;
    }

    pub mod controllers {

        // pub mod process_enum;
        pub mod process_file_contents;
        // pub mod process_function;
        // pub mod process_state_variables;
        // pub mod process_struct;
        pub mod parser;
        // pub mod strip_comments;
        // pub mod structure_to_line_descriptors;
        pub mod sub_main;
    }
}

pub mod implementations {
    // pub mod conditionals;
    // pub mod line_descriptors;
    // pub mod mapping;
}
