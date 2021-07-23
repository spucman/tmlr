use super::{
    data::{ActivityRequest, ActivityResponse},
    TimeularHttpClient,
};
use crate::Result;

const DEFAULT_INTEGRATION: &str = "zei";

impl TimeularHttpClient<'_> {
    pub fn create_activity(
        &self,
        token: String,
        name: String,
        color: String,
        space_id: String,
    ) -> Result<ActivityResponse> {
        self.post(
            "/activities",
            token,
            &ActivityRequest {
                name,
                color,
                integration: DEFAULT_INTEGRATION.to_string(),
                space_id,
            },
            "creating an activity".to_owned(),
        )
    }
}
