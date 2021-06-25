use super::TimeularHttpClient;
use crate::{
    error::Error::{ParseJsonError, TimeularApiError},
    timeular::data::*,
    Result,
};

impl TimeularHttpClient<'_> {
    pub fn create_activity(&self, token: String) -> Result<String> {
        let url = self.uri("/activities");
        println!("{}", token);
        /*
        let resp = self
            .client
            .post(url.to_owned())
            .headers(construct_headers(&token))
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
        */
        Ok("".to_owned())
    }
}
