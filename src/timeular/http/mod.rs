use crate::error::Error;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
};
use std::time::Duration;

pub mod activity;
pub mod auth;
mod data;
pub mod space;

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

    fn construct_headers(token: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(v) = token {
            let bearer_token = format!("Bearer {}", v);
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&bearer_token).expect("A valid header"),
            );
        }
        headers
    }

    fn create_default_error(url: String, resp: Response) -> Error {
        Error::TimeularApiError(
            url.to_owned(),
            format!(
                "status: {}, message: {}",
                resp.status().to_string(),
                resp.text().unwrap_or_else(|_| "".to_string())
            ),
        )
    }
}
