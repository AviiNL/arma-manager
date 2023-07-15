use api_schema::{request::*, response::*, *};
use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Deserializer};
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

#[derive(Debug, Clone)]
pub struct AuthorizedApi {
    url: &'static str,
    token: ApiToken,
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }
    pub async fn register(&self, credentials: &RegisterUserSchema) -> Result<FilteredUser> {
        let url = format!("{}/auth/register", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        into_json(response).await
    }
    pub async fn login(&self, credentials: &LoginUserSchema) -> Result<AuthorizedApi> {
        let url = format!("{}/auth/login", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        let token = into_json(response).await?;
        Ok(AuthorizedApi::new(self.url, token))
    }
}

impl AuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }

    fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token.token)
    }

    async fn send<T>(&self, req: Request) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let req = req.header("Authorization", &self.auth_header_value());

        let response = req.send().await?;
        into_json(response).await
    }

    pub async fn logout(&self) -> Result<SimpleResponse> {
        let url = format!("{}/auth/logout", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn user_info(&self) -> Result<FilteredUser> {
        let url = format!("{}/users/me", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn health_check(&self) -> Result<SimpleResponse> {
        let url = format!("{}/auth/health_check", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn last_status(&self) -> Result<Status> {
        let url = format!("{}/status", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn update_arma(&self) -> Result<SimpleResponse> {
        let url = format!("{}/arma/update", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn cancel_update_arma(&self) -> Result<SimpleResponse> {
        let url = format!("{}/arma/cancel_update", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn start_arma(&self) -> Result<SimpleResponse> {
        let url = format!("{}/arma/start", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn stop_arma(&self) -> Result<SimpleResponse> {
        let url = format!("{}/arma/stop", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn restart_arma(&self) -> Result<SimpleResponse> {
        let url = format!("{}/arma/restart", self.url);
        self.send(Request::get(&url)).await
    }

    pub async fn get_log(&self, channel: impl Into<String>) -> Result<LogResponse> {
        let url = format!("{}/logs/{}", self.url, channel.into());
        self.send(Request::get(&url)).await
    }

    pub async fn post_preset(&self, preset: &CreatePresetSchema) -> Result<Preset> {
        let url = format!("{}/presets", self.url);
        self.send(Request::post(&url).json(preset)?).await
    }

    pub fn token(&self) -> &ApiToken {
        &self.token
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(api_schema::response::Error),
}

impl From<api_schema::response::Error> for Error {
    fn from(e: api_schema::response::Error) -> Self {
        Self::Api(e)
    }
}

async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    // the response looks like { "status": "[ok|error]", [random_key_name]: [value] }

    // ensure we've got 2xx status
    if response.ok() {
        let response: Value = response.json().await?;
        let response = response.as_object().unwrap();

        let response = response.iter().find(|(k, v)| k != &"status").unwrap().1;

        Ok(T::deserialize(response).unwrap())
    } else {
        Err(response.json::<api_schema::response::Error>().await?.into())
    }
}
