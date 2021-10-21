use serde::Deserialize;

#[derive(Debug)]
pub enum MavenError<'a> {
    Args(getargs::Error<'a>),
    Http(reqwest::Error),
    Json(serde_json::Error),
}

impl<'a> From<getargs::Error<'a>> for MavenError<'a> {
    fn from(err: getargs::Error<'a>) -> Self {
        MavenError::Args(err)
    }
}

impl<'a> From<reqwest::Error> for MavenError<'a> {
    fn from(err: reqwest::Error) -> Self {
        MavenError::Http(err)
    }
}

impl<'a> From<serde_json::Error> for MavenError<'a> {
    fn from(err: serde_json::Error) -> Self {
        MavenError::Json(err)
    }
}

pub type MavenResult<'a, T> = Result<T, MavenError<'a>>;

#[derive(Clone, Debug)]
pub struct MavenSearchArgs<'a> {
    pub show_version: bool,
    pub show_help: bool,
    pub format: &'a str,
    pub search_term: Option<&'a String>,
}

impl<'a> Default for MavenSearchArgs<'a> {
    fn default() -> Self {
        MavenSearchArgs {
            show_version: false,
            show_help: false,
            format: "",
            search_term: None,
        }
    }
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