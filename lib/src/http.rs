use crate::types::{Doc, HttpResponse, MavenResult};

pub fn search<'a>(query: &str) -> MavenResult<'a, Vec<Doc>> {
    let url = format!(
        "https://search.maven.org/solrsearch/select?start=0&rows=1&q={}",
        query,
    );

    let resp: HttpResponse = reqwest::blocking::get(url)?.json()?;

    Ok(resp.response.docs)
}
