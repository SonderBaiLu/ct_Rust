use std::fmt;
use std::str::FromStr;
use clap::Parser;

use crate::cli::verify_input_file;

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "将字符串编码为 base64 字符串 并输出")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "将 base64 字符串解码为原始字符串 并输出")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug,Clone,Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err  = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url-safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid base64 format: {}", s)),
        }
    }
}
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "url-safe"),
        }
    }
}
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url-safe",
        }
    }
}