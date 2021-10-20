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
            format: "maven",
            search_term: None,
        }
    }
}

#[derive(Debug)]
pub struct MavenCoordinate {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
}

impl MavenCoordinate {
    pub fn new(gav: String) -> MavenCoordinate {
        let gav_parts: Vec<&str> = gav.split(':').collect();
        if gav_parts.len() == 2 {
            MavenCoordinate {
                group_id: gav_parts[0].to_string(),
                artifact_id: gav_parts[1].to_string(),
                version: "".to_string(),
            }
        } else {
            MavenCoordinate {
                group_id: "".to_string(),
                artifact_id: gav,
                version: "".to_string(),
            }
        }
    }
}

#[derive(Debug)]
struct SearchResult {
    pub coordinates: Vec<MavenCoordinate>,
}

#[derive(Debug, Deserialize)]
pub struct Doc {
    id: String,
    g: String,
    a: String,
    v: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub docs: Vec<Doc>,
}

#[derive(Debug, Deserialize)]
pub struct HttpResponse {
    pub response: SearchResponse,
}
