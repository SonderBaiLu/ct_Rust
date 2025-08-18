use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 帕斯卡命名法（PascalCase） 与骆驼命名法类似。只不过骆驼命名法是首字母小写，而帕斯卡命名法是首字母大写
struct Player{
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<(), anyhow::Error>{
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize(){
        let record: Player = result?;
        ret.push(record);
    }
    let jsons = serde_json::to_string_pretty(&ret)?;
    fs::write(output, jsons)?;
    Ok(())
}


