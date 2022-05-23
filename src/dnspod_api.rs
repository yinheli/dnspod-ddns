use std::{collections::HashMap, fmt::Display};

use anyhow::Ok;

pub struct DnspodApi {
    client: reqwest::Client,
    token: String,
    domain: String,
}

#[derive(Debug)]
pub enum Record {
    A(String, String),
    NotFound,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(id, value) => write!(f, "id:{}, value:{}", id, value),
            Self::NotFound => write!(f, "not found"),
        }
    }
}

impl DnspodApi {
    pub fn new(token: String, domain: String) -> Self {
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(std::time::Duration::from_secs(3))
            .timeout(std::time::Duration::from_secs(5))
            .pool_max_idle_per_host(2)
            .pool_idle_timeout(std::time::Duration::from_secs(60))
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        Self {
            client,
            token,
            domain,
        }
    }

    pub async fn get_record(&self, sub_domain: &str) -> Result<Record, anyhow::Error> {
        let endpoint = self.endpoint("/Record.List");

        let mut params = HashMap::new();
        params.insert("sub_domain", sub_domain);
        params.insert("domain", &self.domain);
        params.insert("login_token", &self.token);
        params.insert("format", "json");

        let resp: serde_json::Value = self
            .client
            .post(endpoint)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        let status_code = resp["status"]["code"].as_str().unwrap();

        match status_code {
            "1" => {
                let id = resp["records"][0]["id"].as_str().unwrap();
                let value = resp["records"][0]["value"].as_str().unwrap();
                Ok(Record::A(id.to_string(), value.to_string()))
            }
            "10" => Ok(Record::NotFound),
            _ => Err(anyhow::anyhow!("{}", status_code)),
        }
    }

    pub async fn update_record(
        &self,
        sub_domain: &str,
        record_id: &str,
        ip: &str,
    ) -> Result<(), anyhow::Error> {
        let endpoint = self.endpoint("/Record.Modify");

        let mut params = HashMap::new();
        params.insert("sub_domain", sub_domain);
        params.insert("record_id", record_id);
        params.insert("domain", &self.domain);
        params.insert("record_type", "A");
        params.insert("record_line", "默认");
        params.insert("value", ip);
        params.insert("mx", "1");
        params.insert("login_token", &self.token);
        params.insert("format", "json");

        let resp: serde_json::Value = self
            .client
            .post(endpoint)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        let status_code = resp["status"]["code"].as_str().unwrap();
        if status_code != "1" {
            return Err(anyhow::anyhow!("{}", status_code));
        }

        Ok(())
    }

    fn endpoint(&self, api: &str) -> String {
        "https://dnsapi.cn".to_string() + api
    }
}
