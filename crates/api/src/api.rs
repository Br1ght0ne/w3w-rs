use std::fmt;

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Error { error: ErrorResponse },
    Ok(T),
}

impl<T> From<ApiResponse<T>> for Result<T, Error> {
    fn from(val: ApiResponse<T>) -> Self {
        match val {
            ApiResponse::Error { error } => Err(Error::Api(error)),
            ApiResponse::Ok(inner) => Ok(inner),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ErrorCode {
    // 400 Bad Request
    BadWords,
    BadCoordinates,
    BadLanguage,
    BadFormat,
    BadClipToPolygon,
    MissingWords,
    MissingInput,
    MissingBoundingBox,
    DuplicateParameter,

    // 401 Unauthorized
    MissingKey,
    InvalidKey,

    // 404 Not Found
    NotFound,

    // 405 Method Not Allowed
    MethodNotAllowed,

    // 500 Internal Server Error
    InternalServerError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coords {
    pub country: String,
    pub square: Square,
    pub nearest_place: Option<String>,
    pub coordinates: GeoCoords,
    pub words: String,
    pub language: String,
    pub map: url::Url,
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.coordinates)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Square {
    pub southwest: GeoCoords,
    pub northeast: GeoCoords,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeoCoords {
    pub lat: f64,
    pub lng: f64,
}

impl From<GeoCoords> for geo_types::Point<f64> {
    fn from(val: GeoCoords) -> Self {
        geo_types::Point::new(val.lng, val.lat)
    }
}

impl fmt::Display for GeoCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.lat, self.lng)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvailableLanguages {
    pub languages: Vec<Language>,
}

impl fmt::Display for AvailableLanguages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.languages
                .iter()
                .map(|lang| lang.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    code: String,
    name: String,
    native_name: String,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
