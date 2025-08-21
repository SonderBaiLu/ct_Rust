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
            let output = if let Some(output) = opts.output{
                output.clone()
            } else {
                format!("output.{:?}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}


















