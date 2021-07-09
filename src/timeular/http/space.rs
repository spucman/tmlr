use super::{data::space::SpaceListResponse, TimeularHttpClient};
use crate::{
    error::Error::{NoDefaultSpaceFound, ParseJsonError, TimeularApiError},
    Result,
};

impl TimeularHttpClient<'_> {
    pub fn list_spaces(&self, token: String) -> Result<SpaceListResponse> {
        let url = self.uri("/space");

        let resp = self
            .client
            .get(url.to_owned())
            .bearer_auth(token)
            .send()
            .map_err(|e| TimeularApiError(url.to_owned(), e.to_string()))?;

        if !resp.status().is_success() {
            return Err(TimeularHttpClient::create_default_error(
                url.to_owned(),
                resp,
            ));
        }

        let result: SpaceListResponse = resp
            .json()
            .map_err(|_| ParseJsonError("fetching spaces".to_owned()))?;
        Ok(result)
    }

    pub fn get_default_space_id(&self, token: String) -> Result<String> {
        Ok(self
            .list_spaces(token)?
            .default_space()
            .ok_or_else(|| NoDefaultSpaceFound)?
            .id
            .to_owned())
    }
}
