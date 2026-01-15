use clap::{Parser};
use csv::Reader;
use serde_json;
use rcli::{Opts, Player, SubCommand};

fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {  // 命名尽量要简单
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret: Vec<Player> = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result?;
                ret.push(record);
            }
            let json = serde_json::to_string_pretty(&ret)?;
            std::fs::write(&opts.output, json)?;
        }
    }
    Ok(())
}
