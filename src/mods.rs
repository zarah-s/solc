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
    pub mod line_descriptors;
    pub mod mapping;
}
