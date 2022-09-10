use crate::dnspod_api::{DnspodApi, Record};

use anyhow::Error;
use chrono::Local;
use clap::Parser;
use log::{error, info, trace, warn};
use std::io::Write;
use tokio::time::{self, Duration};

mod args;
mod dnspod_api;

async fn get_ip() -> Result<String, Error> {
    let result = reqwest::get("http://ns1.dnspod.net:6666")
        .await?
        .text()
        .await?;

    Ok(result.trim().to_owned())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = args::Args::parse();

    let mut level = log::LevelFilter::Info;

    if args.verbose {
        level = log::LevelFilter::Trace;
    }

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter_module("dnspod_ddns", level)
        .init();

    let api = DnspodApi::new(args.token, args.domain);

    let record = api.get_record(&args.sub_domain).await;

    let mut record_id = "".to_string();
    let mut record_value = "".to_string();

    match record {
        Ok(Record::A(id, value)) => {
            record_id = id;
            record_value = value;
            info!("record: {}", record_value);
        }
        Ok(Record::NotFound) => {
            error!("record not found");
        }
        Err(e) => {
            error!("get record, {}", e);
        }
    }

    let mut interval = time::interval(Duration::from_secs(args.interval));

    loop {
        interval.tick().await;

        let my_ip = get_ip().await;

        match my_ip {
            Ok(my_ip) => {
                if my_ip != record_value {
                    info!("update record, {}", my_ip);
                    let result = api
                        .update_record(&args.sub_domain, &record_id, &my_ip)
                        .await;
                    match result {
                        Ok(_) => {
                            info!("update record success");
                            record_value = my_ip;
                        }
                        Err(e) => {
                            warn!("update record failed, {}", e);
                        }
                    }
                } else {
                    trace!("ip not changed");
                }
            }
            Err(e) => {
                warn!("get my ip: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::get_ip;

    #[tokio::test]
    async fn test_get_ip() {
        let ip = get_ip().await;
        assert!(ip.is_ok());
        println!("ip: {:?}", ip.unwrap());
    }
}
