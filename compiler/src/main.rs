use std::{
    env,
    time::{self, SystemTime},
};
mod mods;

use mods::{
    functions::controllers::sub_main::compile_source_code,
    types::types::{
        ContractIdentifier, InterfaceIdentifier, LibraryIdentifier, LineDescriptions, Token,
    },
};
use tokio::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let start_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    /* GET ENVIRONMENT ARGUMENTS */
    let args: Vec<String> = env::args().collect();

    let mut abstract_contracts: Vec<ContractIdentifier> = Vec::new();
    let mut main_contracts: Vec<ContractIdentifier> = Vec::new();
    let mut libraries: Vec<LibraryIdentifier> = Vec::new();
    let mut interfaces: Vec<InterfaceIdentifier> = Vec::new();

    let _ = compile_source_code(
        args,
        &mut abstract_contracts,
        &mut main_contracts,
        &mut libraries,
        &mut interfaces,
    )
    .await?;

    println!(
        "===>>> INTERFACES ===>>>\n\n{:#?}\n\n\n ===>>> LIBRARIES ===>>>\n\n{:#?}\n\n\n ===>>> ABSTRACT CONTRACTS ===>>>\n\n{:#?}\n\n\n ===>>> MAIN CONTRACTS ===>>>\n\n{:#?}",
        interfaces,libraries, abstract_contracts,main_contracts
    );

    let end_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!(
        "Program completed in \x1b[93m{:?}\x1b[0m",
        (end_time.unwrap() - start_time.unwrap())
    );

    Ok(())
}
