use crate::types::{Doc, HttpResponse, MavenCoordinate, MavenResult};

pub fn search<'a>(coordinate: &MavenCoordinate) -> MavenResult<'a, Vec<Doc>> {
    let group_id = coordinate.group_id.as_str();
    let group_id_param = if !group_id.is_empty() {
        format!("{}{}", "g:", group_id)
    } else {
        "".to_string()
    };
    let url = format!(
        "https://search.maven.org/solrsearch/select?rows=2&q={}&a:{}&core=gav",
        group_id_param,
        coordinate.artifact_id,
    );

    let resp: HttpResponse = reqwest::blocking::get(url)?.json()?;

    Ok(resp.response.docs)
}
