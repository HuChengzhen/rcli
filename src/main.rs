mod opts;
mod process;

use opts::{Opts, SubCommand};
use process::process_csv;
use clap::Parser;

// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opt) => {
            println!("{:?}", opt);
            process_csv(&opt.input, &opt.output)?;
        }
    }
    Ok(())
}
