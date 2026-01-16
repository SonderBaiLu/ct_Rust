use clap::Parser;
use rcli::process_csv;
use rcli::{Opts, SubCommand};
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            // 命名尽量要简单
            process_csv(&opts.input, &output, opts.format)?;
        }
    }
    Ok(())
}
