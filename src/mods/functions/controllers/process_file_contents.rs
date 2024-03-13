use std::{fs, process};

use crate::mods::functions::helpers::helpers::print_error;

pub fn process_file_contents(args: Vec<String>, file_contents: &mut String) {
    /* CHECK FOR VALID ARGUMENTS */
    if args.len() < 2 {
        print_error("Mising file path... Run cargo run <file-path>")
    }

    /* VALIDATE FILE FORMAT */
    if args[1].split(".").last().unwrap() != "sol" {
        print_error("Unsupported file... Expected \".sol\" file");
    }

    /* READ FILE TO STRING */
    *file_contents = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        print_error(&format!("Could not read file. {err}"));
        process::exit(1);
    });
}
