use std::io::{BufReader, Read};
use base64::{engine::general_purpose::{STANDARD, URL_SAFE}, Engine as _};
use crate::cli::base64::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        println!("请输入要编码的数据，然后回车，按 Ctrl+Z 结束");
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE.encode(&buf),
        Base64Format::Standard => STANDARD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}
pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        println!("====================================");
        println!("Base64 解码工具");
        println!("====================================");
        println!("请输入要解码的 Base64 字符串，按 Ctrl+Z 然后回车结束输入：");
        Box::new(std::io::stdin())
    } else {
        println!("====================================");
        println!("Base64 解码工具");
        println!("====================================");
        println!("正在从文件 '{}' 读取数据...", input);
        Box::new(std::fs::File::open(input)?) 
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    
    if buf.is_empty() {
        return Err(anyhow::anyhow!("输入数据为空，请提供要解码的 Base64 字符串"));
    }
    
    // 移除输入中的空白字符和换行符
    let input_str = String::from_utf8_lossy(&buf).trim().to_string();
    
    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE.decode(input_str)?,
        Base64Format::Standard => STANDARD.decode(input_str)?,
    };
    
    println!("\n====================================");
    println!("解码结果 ({} 格式):", format);
    println!("====================================");
    
    // 尝试将解码结果转换为字符串，如果失败则显示十六进制
    match String::from_utf8(decoded.clone()) {
        Ok(text) => println!("{}", text),
        Err(_) => {
            println!("解码结果为二进制数据，以十六进制表示：");
            for byte in decoded {
                print!("{:02x} ", byte);
            }
            println!();
        }
    }
    
    println!("====================================");
    Ok(())
}