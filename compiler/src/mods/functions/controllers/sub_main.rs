use crate::mods::functions::controllers::process_file_contents::process_file_contents;

pub async fn compile_source_code(args: Vec<String>) -> Result<(), std::io::Error> {
    let data = process_file_contents(args).await;
    println!("{:?}", data);
    Ok(())
}
