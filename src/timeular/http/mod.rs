use reqwest::blocking::Client;
use std::time::Duration;

mod auth;

#[derive(Clone)]
pub struct TimeularHttpClient<'a> {
    url: &'a str,
    api_version: &'a str,
    client: Client,
}

const TIMEOUT_SECS: u64 = 60;
const BASE_URL: &str = "https://api.timeular.com/api";
const API_VERSION: &str = "v3";
const USER_AGENT: &str = "Tmlr Client";

impl TimeularHttpClient<'_> {
    pub fn new() -> Self {
        Self {
            url: BASE_URL,
            api_version: API_VERSION,
            client: Client::builder()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .user_agent(USER_AGENT)
                .gzip(true)
                .https_only(true)
                .build()
                .expect("Http Client can be created"),
        }
    }

    fn uri(&self, uri: &str) -> String {
        format!("{}/{}{}", self.url, self.api_version, uri)
    }
}
