use crate::types::{Doc, HttpResponse, MavenResult};

const URL: &str = "https://central.sonatype.com/api/internal/browse/components";

pub fn search<'a>(query: &str) -> MavenResult<'a, Vec<Doc>> {
    let request_body = format!(r#"{{"filter":[], "size": 10, "searchTerm": "{query}"}}"#);
    debug!("Going to make a request for : {request_body}");

    let resp: HttpResponse = ureq::post(URL)
        .header("Content-Type", "application/json")
        .send(request_body)?
        .body_mut()
        .read_json()?;

    debug!("response:\n{resp:#?}");
    Ok(resp.components)
}

#[cfg(test)]
mod tests {
    use crate::http::search;

    #[test]
    fn test_search() {
        env_logger::init();

        let docs = search("g:org.apache.wicket a:wicket-core").unwrap();
        assert_eq!(docs.len(), 1);
        let doc = &docs[0];
        assert_eq!(doc.id, "pkg:maven/org.apache.wicket/wicket-core");
        assert_eq!(doc.namespace, "org.apache.wicket");
        assert_eq!(doc.name, "wicket-core");
        assert!(!doc.latestVersionInfo.version.is_empty());
    }
}
