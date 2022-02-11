use crate::types::{Doc, HttpResponse, MavenResult};

const USER_AGENT_NAME: &str = "User-Agent";
const USER_AGENT_VALUE: &str = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:96.0) Gecko/20100101 Firefox/96.0";

pub fn search<'a>(query: &str) -> MavenResult<'a, Vec<Doc>> {
    let url = format!(
        "https://search.maven.org/solrsearch/select?start=0&rows=1&q={}",
        query,
    );

    debug!("Going to make a request to : {:?}", &url);

    let client = reqwest::blocking::Client::new();
    let resp: HttpResponse = client
        .get(url)
        .header(USER_AGENT_NAME, USER_AGENT_VALUE)
        .send()?
        .json()?;

    debug!("response:\n{:?}", &resp);
    Ok(resp.response.docs)
}
