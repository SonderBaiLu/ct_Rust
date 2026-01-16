use std::fmt;
use std::fmt::Formatter;
use std::process::Output;
use std::str::FromStr;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "显示Csv或将Csv转换为其他格式")]
    Csv(CsvOpts),
}
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat{
    Json,
    Yaml,
    Toml,
}

// default_value_t 和 default_value 的区别是：
// default_value_t 是在解析参数时使用的，而 default_value 是在打印帮助信息时使用的
// default_value_t 是在解析参数时使用的默认值，而 default_value 是在打印帮助信息时使用的默认值
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    // 修复bug value_parser 这里之前写成了字符串 但实际需要functionE> 类型
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_output_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    #[arg(long, default_value_t = true)] // 携带short参数与-help冲突
    pub header: bool,
}
// 验证输入文件是否存在 如果存在则返回文件路径 否则返回错误信息
pub fn verify_input_file(csv_file: &str) -> Result<String, &'static str> {
    if std::path::Path::new(csv_file).exists() {
        Ok(csv_file.into())
    } else {
        Err("文件不存在")
    }
}
pub fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("无效的输出格式: {}", v),
        }
    }
}
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}