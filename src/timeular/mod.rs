use crate::timeular::http::TimeularHttpClient;

mod data;
mod http;

#[derive(Clone)]
pub struct Timeular<'a> {
    client: TimeularHttpClient<'a>,
    api_key: String,
    api_secret: String,
    token: String, // mut? possible
}

impl Timeular<'_> {
    fn new(api_key: String, api_secret: String) -> Self {
        Timeular {
            client: TimeularHttpClient::new(),
            api_key,
            api_secret
        }
    }

    fn new_with_token() -> {

    }
}
