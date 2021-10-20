use crate::types::{Doc, HttpResponse, MavenCoordinate, MavenResult};

pub fn search<'a>(coordinate: &MavenCoordinate) -> MavenResult<'a, Vec<Doc>> {
    let group_id = coordinate.group_id.as_str();
    let group_id_param = if !group_id.is_empty() {
        format!("{}{} AND ", "g:", group_id)
    } else {
        "".to_string()
    };
    let artifact_id_param = if !group_id_param.is_empty() {
        format!("a:{}", coordinate.artifact_id)
    } else {
        coordinate.artifact_id.to_string()
    };

    let url = format!(
        "https://search.maven.org/solrsearch/select?start=0&rows=1&q={}{}",
        group_id_param,
        artifact_id_param,
    );

    let resp: HttpResponse = reqwest::blocking::get(url)?.json()?;

    Ok(resp.response.docs)
}
