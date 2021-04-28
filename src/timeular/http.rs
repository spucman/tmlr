use crate::{
    error::Error::{ParseJsonError, TimeularApiError},
    timeular::data::*,
    Result,
};
use reqwest::blocking::Client;
use std::time::Duration;

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

    pub fn login(&self, api_key: String, api_secret: String) -> Result<String> {
        let url = self.uri("/developer/sign-in ");
        let resp = self
            .client
            .post(url.to_owned())
            .json(&LoginRequest {
                api_key,
                api_secret,
            })
            .send()
            .map_err(|e| TimeularApiError(url, e.to_string()))?;

        if !resp.status().is_success() {
            return Err(TimeularApiError(
                url,
                format!(
                    "status: {}, message: {}",
                    resp.status().to_string(),
                    resp.text().unwrap_or_else(|_| "".to_string())
                ),
            ));
        }

        let result: LoginResponse = resp.json().map_err(|_| ParseJsonError)?;
        Ok(result.token)
    }

    pub fn logout(&self, token: &str) -> Result<()> {
        let url = self.uri("/developer/logout");
        self.client
            .post(url.to_owned())
            .bearer_auth(token)
            .send()
            .map_err(|e| TimeularApiError(url, e.to_string()))?;

        Ok(())
    }
}
