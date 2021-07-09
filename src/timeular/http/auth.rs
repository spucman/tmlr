use super::{data::LoginRequest, data::LoginResponse, TimeularHttpClient};
use crate::{
    error::Error::{ParseJsonError, TimeularApiError},
    Result,
};

impl TimeularHttpClient<'_> {
    pub fn login(&self, api_key: String, api_secret: String) -> Result<String> {
        let url = self.uri("/developer/sign-in");
        let resp = self
            .client
            .post(url.to_owned())
            .headers(TimeularHttpClient::construct_headers(None))
            .json(&LoginRequest {
                api_key,
                api_secret,
            })
            .send()
            .map_err(|e| TimeularApiError(url.to_owned(), e.to_string()))?;

        if !resp.status().is_success() {
            return Err(TimeularHttpClient::create_default_error(
                url.to_owned(),
                resp,
            ));
        }

        let result: LoginResponse = resp
            .json()
            .map_err(|_| ParseJsonError("authenticating".to_owned()))?;
        log::debug!("Token: {}", result.token.to_owned());
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
