use super::TimeularHttpClient;
use crate::{
    error::Error::{ParseJsonError, TimeularApiError},
    timeular::data::*,
    Result,
};

impl TimeularHttpClient<'_> {
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
            .map_err(|e| TimeularApiError(url.to_owned(), e.to_string()))?;

        if !resp.status().is_success() {
            return Err(TimeularApiError(
                url.to_owned(),
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
            .map_err(|e| TimeularApiError(url.to_owned(), e.to_string()))?;

        Ok(())
    }
}
