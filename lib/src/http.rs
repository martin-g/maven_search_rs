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

#[cfg(test)]
mod tests {
    use crate::http::search;

    #[test]
    fn test_search() {
        let docs = search("g:org.apache.wicket AND a:wicket-core").unwrap();
        assert_eq!(docs.len(), 1);
        let doc = &docs[0];
        assert_eq!(doc.id, "org.apache.wicket:wicket-core");
        assert_eq!(doc.g, "org.apache.wicket");
        assert_eq!(doc.a, "wicket-core");
        assert!(!doc.latestVersion.is_empty());
    }
}
