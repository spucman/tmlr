use super::{
    data::{TagRequest, TagResponse},
    TimeularHttpClient,
};
use crate::Result;

const DEFAULT_SCOPE: &str = "timeular";

impl TimeularHttpClient<'_> {
    pub fn create_tag(
        &self,
        token: String,
        label: String,
        key: Option<String>,
        space_id: String,
    ) -> Result<TagResponse> {
        self.post(
            "/tags",
            token,
            &TagRequest {
                label,
                key,
                scope: DEFAULT_SCOPE.to_owned(),
                space_id,
            },
            "creating a tag".to_owned(),
        )
    }
}
