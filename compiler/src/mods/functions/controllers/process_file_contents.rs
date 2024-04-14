use tokio::{fs, io};

use crate::mods::functions::helpers::helpers::print_error;

pub async fn process_file_contents(
    args: Vec<String>,
    file_contents: &mut String,
) -> Result<(), io::Error> {
    /* CHECK FOR VALID ARGUMENTS */

    if args.len() < 2 {
        print_error("Mising file path... Run cargo run <file-path>")
    }

    /* VALIDATE FILE FORMAT */
    if args[1].split(".").last().unwrap() != "sol" {
        print_error("Unsupported file... Expected \".sol\" file");
    }

    /* READ FILE TO STRING */
    *file_contents = fs::read_to_string(&args[1]).await?;
    Ok(())
}
