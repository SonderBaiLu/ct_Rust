use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

// #[derive(Debug, Deserialize, Serialize)]
// struct Player{
//     #[serde(rename = "Name")]
//     name: String,
//     #[serde(rename = "Position")]
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     #[serde(rename = "Nationality")]
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }
fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}


















