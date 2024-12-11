use serde::Deserialize;

#[derive(Debug)]
pub enum MavenError<'a> {
    Args(getargs::Error<&'a str>),
    Http(String),
    Json(serde_json::Error),
    IO(std::io::Error),
}

impl<'a> From<getargs::Error<&'a str>> for MavenError<'a> {
    fn from(err: getargs::Error<&'a str>) -> Self {
        MavenError::Args(err)
    }
}

impl From<ureq::Error> for MavenError<'_> {
    fn from(err: ureq::Error) -> Self {
        MavenError::Http(err.to_string())
    }
}

impl From<serde_json::Error> for MavenError<'_> {
    fn from(err: serde_json::Error) -> Self {
        MavenError::Json(err)
    }
}

impl From<std::io::Error> for MavenError<'_> {
    fn from(err: std::io::Error) -> Self {
        MavenError::IO(err)
    }
}

pub type MavenResult<'a, T> = Result<T, MavenError<'a>>;

#[derive(Clone, Debug, Default)]
pub struct MavenSearchArgs<'a> {
    pub show_version: bool,
    pub show_help: bool,
    pub format: &'a str,
    pub search_term: Option<&'a str>,
    pub check_for_update: bool,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Doc {
    pub id: String,
    pub g: String,
    pub a: String,
    #[serde(default)]
    pub latestVersion: String,
    #[serde(default)]
    pub v: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub docs: Vec<Doc>,
}

#[derive(Debug, Deserialize)]
pub struct HttpResponse {
    pub response: SearchResponse,
}
