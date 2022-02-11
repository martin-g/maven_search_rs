use crate::types::{Doc, HttpResponse, MavenResult};

pub fn search<'a>(query: &str) -> MavenResult<'a, Vec<Doc>> {
    let url = format!(
        "https://search.maven.org/solrsearch/select?start=0&rows=1&q={}",
        query,
    );

    debug!("Going to make a request to : {:?}", &url);

    let resp: HttpResponse = ureq::get(url.as_str()).call()?.into_json()?;

    debug!("response:\n{:?}", &resp);
    Ok(resp.response.docs)
}
