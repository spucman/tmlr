use crate::timeular::http::TimeularHttpClient;

mod data;
mod http;

#[derive(Clone)]
pub struct TimeularCredentials {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Clone)]
pub struct TimeularAuth {
    pub credentials: TimeularCredentials,
    pub token: Option<String>,
}

impl TimeularAuth {
    pub fn new(api_key: String, api_secret: String) -> Self {
        TimeularAuth {
            credentials: TimeularCredentials {
                api_key,
                api_secret,
            },
            token: None,
        }
    }
}

#[derive(Clone)]
pub struct Timeular<'a> {
    client: TimeularHttpClient<'a>,
    auth_data: TimeularAuth,
}

impl Timeular<'_> {
    pub fn new(auth_data: TimeularAuth) -> Self {
        Timeular {
            client: TimeularHttpClient::new(),
            auth_data,
        }
    }
}
