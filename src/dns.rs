/// DNS functions according to RFC 1035
/// https://www.rfc-editor.org/info/rfc1035/
use clap::Args;

use crate::dns::{header::Header, question::Question};

pub mod header;
pub mod question;
mod misc;
pub mod resource_record;

#[derive(Args)]
pub struct DnsArgs {
    #[arg(long)]
    dns_server: String,

    #[arg(long)]
    domain: String,
}

struct Message {
    header: Header,
    question: Question,
}

pub fn get_dns_info(args: &DnsArgs) {
    let dns_server_addr = &args.dns_server;
    let domain = &args.domain;
}
