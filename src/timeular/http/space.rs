use super::{data::space::SpaceListResponse, TimeularHttpClient};
use crate::{error::Error::NoDefaultSpaceFound, Result};

impl TimeularHttpClient<'_> {
    pub fn list_spaces(&self, token: String) -> Result<SpaceListResponse> {
        self.get(token, "/space", "fetching spaces".to_owned())
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
