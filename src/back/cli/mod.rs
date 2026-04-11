pub mod base64;
pub mod csv;
pub mod genpass;

pub use self::{base64::Base64SubCommand, csv::CsvOpts, genpass::GenPassOpts};
use anyhow::{anyhow, Ok};
use clap::Parser;

// 这里的csv跟cargo.toml文件中定义的csv名字是有冲突，为了避免歧义 使用self::csv
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
    #[command(name = "genpass", about = "生成随机密码")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}
// 验证输入文件是否存在 如果存在则返回文件路径 否则返回错误信息
fn verify_input_file(csv_file: &str) -> anyhow::Result<String> {
    // if input is "-" or file exists
    if csv_file == "-" || std::path::Path::new(csv_file).exists() {
        Ok(csv_file.into())
    } else {
        Err(anyhow!("文件不存在或路径错误：{}", csv_file))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        // 测试标准输入符号"-"：成功，且返回值为"-"
        let stdin_test = verify_input_file("-");
        assert!(stdin_test.is_ok());
        assert_eq!(stdin_test.unwrap(), "-".to_string());

        // 测试存在的文件（Cargo.toml）：成功，且返回值为文件名
        let exist_file = verify_input_file("Cargo.toml");
        assert!(exist_file.is_ok());
        assert_eq!(exist_file.unwrap(), "Cargo.toml".to_string());

        // 测试不存在的文件：失败，且错误信息包含指定关键词
        let non_exist_file = verify_input_file("nonexistent.csv");
        assert!(non_exist_file.is_err());
        assert!(non_exist_file
            .unwrap_err()
            .to_string()
            .contains("文件不存在或路径错误"));
    }
}

