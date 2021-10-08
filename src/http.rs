use crate::types::{Doc, HttpResponse, MavenCoordinate, MavenResult};

pub fn search<'a>(coordinate: &MavenCoordinate) -> MavenResult<'a, Vec<Doc>> {
    let url = format!(
        "https://search.maven.org/solrsearch/select?rows=2&q=a:{}&core=gav",
        coordinate.artifact_id
    );

    let resp = reqwest::blocking::get(url)?.text()?;

    get_docs(resp)
}

fn get_docs<'a>(response: String) -> MavenResult<'a, Vec<Doc>> {
    let v: HttpResponse = serde_json::from_str(response.as_str())?;

    Ok(v.response.docs)
}
