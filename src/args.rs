/// simple dnspod ddns client, more info: http://github.com/yinheli/dnspod-ddns
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// domain, eg: example.com
    #[clap(short, long)]
    pub domain: String,

    /// sub domain, eg: www
    #[clap(short, long)]
    pub sub_domain: String,

    /// dnspod api key / token, eg: 12345,aeaae98e8fbee8369f93ec46c4384aed
    #[clap(short, long, env = "DNSPOD_API_KEY")]
    pub token: String,

    /// check interval seconds, eg: 10
    /// default: 10
    /// min: 5
    /// max: 3600
    #[clap(short, long, env = "DNSPOD_INTERVAL", default_value = "10", value_parser = clap::value_parser!(u64).range(5..=3600))]
    pub interval: u64,

    /// verbose log
    #[clap(long)]
    pub verbose: bool,
}
