use std::path::Path;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name ="rcli", version = "1.0", author, about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "显示 CSV，将 CSV 转换为其他格式 ")]
    Csv(CsvOpts),
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long,default_value = "output.json")] // output.json.into()
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true )]
    pub header: bool,
}
pub fn verify_input_file(filename:&str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("文件不存在".into())
    }
}