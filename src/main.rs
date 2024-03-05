use std::env;

use crate::dnspod_api::{DnspodApi, Record};

use anyhow::Error;
use clap::Parser;
use tokio::time::{self, Duration};
use tracing::{error, info, trace, warn};

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
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "error,dnspod_ddns=debug");
    }
    let _ = tracing_subscriber::fmt::init();

    let args = args::Args::parse();

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
