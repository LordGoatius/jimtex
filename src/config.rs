use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    pub command: SubCommand
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    StartKernel,
    ConnectKernel {
        #[arg(short, long, required = false)]
        port: u16
    }
}
