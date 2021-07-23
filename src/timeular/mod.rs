use crate::{error::Error::AuthenticationInformationMissingError, Result};
use http::TimeularHttpClient;
use rand::Rng;

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

trait AuthenticatedCall {
    fn invoke(&self, token: String) -> Result<()>;
}

impl Timeular<'_> {
    pub fn new(auth_data: TimeularAuth) -> Result<Self> {
        let tmlr_client = TimeularHttpClient::new();

        let auth = match &auth_data.token {
            Some(_) => auth_data,
            None => {
                log::debug!("Fetching new authentication token.");
                let token = tmlr_client.login(
                    auth_data.credentials.api_key.to_owned(),
                    auth_data.credentials.api_secret.to_owned(),
                )?;

                TimeularAuth {
                    credentials: auth_data.credentials.to_owned(),
                    token: Some(token),
                }
            }
        };

        Ok(Timeular {
            client: tmlr_client,
            auth_data: auth,
        })
    }

    pub fn create_activity(
        &self,
        name: String,
        color: Option<String>,
        space_id: Option<String>,
    ) -> Result<(String, String)> {
        match &self.auth_data.token {
            Some(v) => {
                let ac = self.client.create_activity(
                    v.to_owned(),
                    name.to_owned(),
                    color.unwrap_or_else(|| create_random_color()),
                    match space_id {
                        Some(v) => v,
                        None => {
                            let id = self.client.get_default_space_id(v.to_owned())?;
                            log::debug!("No space provided - getting default space ({})", id);
                            id
                        }
                    },
                )?;
                Ok((ac.id, ac.name))
            }
            None => Err(AuthenticationInformationMissingError),
        }
    }

    pub fn create_tag(
        &self,
        label: String,
        key: Option<String>,
        space_id: Option<String>,
    ) -> Result<i64> {
        match &self.auth_data.token {
            Some(v) => {
                let tag = self.client.create_tag(
                    v.to_owned(),
                    label,
                    key,
                    match space_id {
                        Some(v) => v,
                        None => {
                            let id = self.client.get_default_space_id(v.to_owned())?;
                            log::debug!("No space provided - getting default space ({})", id);
                            id
                        }
                    },
                )?;
                Ok(tag.id)
            }
            None => Err(AuthenticationInformationMissingError),
        }
    }
}

impl Drop for Timeular<'_> {
    fn drop(&mut self) {
        if let Some(t) = &self.auth_data.token {
            log::debug!("Releasing authentication token.");
            if let Err(e) = self.client.logout(t) {
                log::debug!("Unable to release authentication token: {}", e.to_string());
            }
        }
    }
}

fn create_random_color() -> String {
    let mut gen = rand::thread_rng();
    let r = gen.gen_range(0..255);
    let g = gen.gen_range(0..255);
    let b = gen.gen_range(0..255);
    let color = format!("#{:02x}{:02x}{:02x}", r, g, b);
    log::debug!("Generating random color {}", color);
    color
}
