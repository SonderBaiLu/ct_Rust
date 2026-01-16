use std::collections::HashMap;
use chrono::format::Item::Error;
use chrono::Utc;
use csv::{Reader, StringRecord};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use toml::Value::Datetime;
use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "Birthday")]
    pub birthday: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "Constellation Level")]
    pub constellation_level: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut ret = Vec::with_capacity(128);
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用headers 的迭代器
        // record.iter() -> 使用record 的迭代器
        // 这里使用zip 是为了将headers 和 record 中的元素一一对应起来
        // 例如：headers = ["Name", "Role", "Birthday"]
        // record = ["Sonde", "Tank", "2000-01-01"]
        // 那么zip 后就是 [("Name", "Sonde"), ("Role", "Tank"), ("Birthday", "2000-01-01")]
        // collect::<Value>() -> 将zip 后的迭代器转换为 Value 类型
        // 这里的 Value 类型是 serde_json 中的类型
        // 它可以表示任意的 JSON 数据类型
        // 例如：{"Name": "Sonde", "Role": "Tank", "Birthday": "2000-01-01"}
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?,
    };
    let json = serde_json::to_string_pretty(&ret)?;
    // 获取当前时间
    let now = Utc::now();
    // 格式化时间为字符串
    let now_str = now.format("%Y年%m月%d日%H时%M分%S秒").to_string();
    std::fs::write(format!("{}.{}", now_str, format), content)?;
    Ok(())
}
