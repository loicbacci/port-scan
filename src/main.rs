use clap::{Parser, Subcommand};

use crate::{
    dns::question::{QCLASS, QTYPE, Question},
    port_scan::{PortScanArgs, port_scan},
};

mod dns;
mod port_scan;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Port scan
    PortScan(PortScanArgs),
}

fn main() {
    let cli = Cli::parse();

    // // Find which subcommand
    // match &cli.command {
    //     Commands::PortScan(args) => {
    //         port_scan(args);
    //     }
    // }

    let q = Question::new(vec![("test.abc", QTYPE::A, QCLASS::IN)]);
    println!("Question:\n{}", q);

    println!("\nQuestion (debug):\n{:#?}", q);
}
