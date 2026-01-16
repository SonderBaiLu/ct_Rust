use rcli::process_csv;
use clap::{Parser};
use rcli::{Opts, SubCommand};

fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {  // 命名尽量要简单
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
