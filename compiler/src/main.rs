use std::{
    env,
    time::{self, SystemTime},
};
mod mods;

use mods::functions::controllers::sub_main::compile_source_code;

#[tokio::main]
async fn main() {
    let start_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    /* GET ENVIRONMENT ARGUMENTS */
    let args: Vec<String> = env::args().collect();

    let _ = compile_source_code(args).await.unwrap();

    let end_time = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!(
        "Program completed in \x1b[93m{:?}\x1b[0m",
        (end_time.unwrap() - start_time.unwrap())
    );
}
