use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<(), anyhow::Error>{
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    println!("{:?}", headers);
    for result in reader.records(){
        let record = result?;
        let json_value = headers.iter()
            .zip(record.iter()).collect::<Value>();
        ret.push(record);
        println!("{:?}", json_value);
    }
    // let jsons = serde_json::to_str ng_pretty(&ret)?;
    // fs::write(output, jsons)?;
    Ok(())
}


