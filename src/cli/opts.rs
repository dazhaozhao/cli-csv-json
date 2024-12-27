use super::*;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, default_value = "assets/input.csv", value_parser = verify_input)]
    pub input: String,

    #[arg(short, long, default_value = "assets/output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = false)]
    pub pretty: bool,
}

fn verify_input(file_name: &str) -> Result<String, String> {
    let _ =
        std::fs::metadata(file_name).map_err(|_| format!("{} file doesn't exist", file_name))?;
    Ok(file_name.into())
}
