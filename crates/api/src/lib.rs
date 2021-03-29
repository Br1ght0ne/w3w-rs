use std::io;

use derivative::Derivative;
use serde::de::DeserializeOwned;
use ureq::OrAnyStatus;

pub mod api;

use crate::api::{ApiResponse, GeoCoords};
pub use crate::api::{AvailableLanguages, Coords};

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[error("API returned an error: {0:?}")]
    Api(crate::api::ErrorResponse),
    #[error("HTTP transport error")]
    HttpTransport(#[from] ureq::Transport),
    #[error("JSON error")]
    Json(#[from] io::Error),
    #[error("URL error")]
    Url(#[from] url::ParseError),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Client {
    #[derivative(Debug = "ignore")]
    key: String,
    pub base_url: url::Url,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let client = Self {
            key: key.to_string(),
            base_url: url::Url::parse("https://api.what3words.com/v3/").unwrap(),
        };
        tracing::debug!(%client.base_url, "creating new client");
        client
    }

    #[tracing::instrument(skip(self))]
    fn prepare_request(&self, path: &str) -> Result<ureq::Request, Error> {
        let url = self.base_url.join(path)?;
        let request = ureq::get(url.as_str()).query("format", "json");
        tracing::trace!(url = ?request.url());
        Ok(request.query("key", &self.key))
    }

    #[tracing::instrument(skip(self, request), err)]
    fn send<T: DeserializeOwned>(&self, request: ureq::Request) -> Result<ApiResponse<T>, Error> {
        let response = request
            .call()
            .or_any_status()
            .map_err(Error::HttpTransport)?;
        let (status, status_text) = (response.status(), response.status_text());
        tracing::trace!(response.status = status, response.status_text = status_text);
        response.into_json().map_err(Error::Json)
    }

    #[tracing::instrument(skip(self), err)]
    pub fn convert_to_coordinates(&self, words: &str) -> Result<Coords, Error> {
        let request = self
            .prepare_request("convert-to-coordinates")?
            .query("words", words);
        self.send(request)?.into()
    }

    #[tracing::instrument(skip(self), err)]
    pub fn convert_to_3wa(&self, coordinates: &GeoCoords) -> Result<Coords, Error> {
        let request = self
            .prepare_request("convert-to-3wa")?
            .query("coordinates", &coordinates.to_string());
        self.send(request)?.into()
    }

    #[tracing::instrument(skip(self), err)]
    pub fn available_languages(&self) -> Result<AvailableLanguages, Error> {
        let request = self.prepare_request("available-languages")?;
        self.send(request)?.into()
    }
}
