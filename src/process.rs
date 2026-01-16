use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Deserialize)]
pub struct Player{
    #[serde(rename= "Name")]
    pub name: String,
    #[serde(rename= "Role")]
    pub role: String,
    #[serde(rename= "Birthday")]
    pub birthday: String,
    #[serde(rename= "Region")]
    pub region: String,
    #[serde(rename= "Constellation Level")]
    pub constellation_level: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret: Vec<Player> = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;
    Ok(())
}
