mod cli;
mod process;

pub use cli::{Base64SubCommand, Opts, SubCommand};
pub use process::{process_csv, process_genpass, process_encode, process_decode};
