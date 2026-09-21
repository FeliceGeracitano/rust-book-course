use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use url::Url;

pub const MAX_BODY_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HttpJsonError {
    #[error("request timed out")]
    Timeout,
    #[error("unexpected status {0}")]
    Status(u16),
    #[error("transport failure: {0}")]
    Transport(String),
    #[error("bad response body: {0}")]
    Body(String),
}

pub(crate) enum Fetched<T> {
    Found(T),
    NotFound,
}

/// Bounded JSON GET shared by every adapter: one timeout policy (set on the
/// client), one body cap, one place that decides what "not found" means.
pub(crate) async fn get_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: Url,
) -> Result<Fetched<T>, HttpJsonError> {
    let response = client.get(url).send().await.map_err(classify)?;
    if response.status() == StatusCode::NOT_FOUND {
        return Ok(Fetched::NotFound);
    }
    if !response.status().is_success() {
        return Err(HttpJsonError::Status(response.status().as_u16()));
    }
    if response
        .content_length()
        .is_some_and(|length| length > MAX_BODY_BYTES as u64)
    {
        return Err(HttpJsonError::Body("response too large".into()));
    }
    let bytes = response.bytes().await.map_err(classify)?;
    if bytes.len() > MAX_BODY_BYTES {
        return Err(HttpJsonError::Body("response too large".into()));
    }
    serde_json::from_slice(&bytes)
        .map(Fetched::Found)
        .map_err(|error| HttpJsonError::Body(error.to_string()))
}

fn classify(error: reqwest::Error) -> HttpJsonError {
    if error.is_timeout() {
        HttpJsonError::Timeout
    } else {
        HttpJsonError::Transport(error.to_string())
    }
}
