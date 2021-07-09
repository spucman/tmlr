use super::{
    data::{ActivityRequest, ActivityResponse},
    TimeularHttpClient,
};
use crate::{
    error::Error::{ParseJsonError, TimeularApiError},
    Result,
};

const DEFAULT_INTEGRATION: &str = "zei";

impl TimeularHttpClient<'_> {
    pub fn create_activity(
        &self,
        token: String,
        name: String,
        color: String,
        space_id: String,
    ) -> Result<ActivityResponse> {
        let url = self.uri("/activities");

        let resp = self
            .client
            .post(url.to_owned())
            .headers(TimeularHttpClient::construct_headers(Some(&token)))
            .json(&ActivityRequest {
                name,
                color,
                integration: DEFAULT_INTEGRATION.to_string(),
                space_id,
            })
            .send()
            .map_err(|e| TimeularApiError(url.to_owned(), e.to_string()))?;

        if !resp.status().is_success() {
            return Err(TimeularHttpClient::create_default_error(
                url.to_owned(),
                resp,
            ));
        }

        let result: ActivityResponse = resp.json().map_err(|e| {
            log::info!("{:?}", e);
            ParseJsonError("creating activities".to_owned())
        })?;
        Ok(result)
    }
}
