use std::path::Path;
use std::str::FromStr;
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
#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)] // output.yaml.into()
    pub output: Option<String>,
    #[arg(long, value_parser= parser_format,default_value = "json")]
    pub format: OutputFormat,
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

fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            _ => anyhow::bail!("不支持的输出格式: '{}'. 支持的格式有: json, yaml, toml",s)
        }
    }
}
// 报错临时取消
// impl fmt::Display for OutputFormat {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", Into::<&'static str>::into(*self))
//     }
// }