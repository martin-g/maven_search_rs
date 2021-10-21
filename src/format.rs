use crate::types::Doc;

pub fn format(results: Vec<Doc>, fmt: &str) -> () {
    match fmt {
        "gradle" => { gradle(results) }
        "gradle.kts" | "gradle_kts" | "gradlekts" => { gradle_kts(results) }
        "sbt" => { sbt(results) }
        "lein" => { lein(results) }
        "ivy" => { ivy(results) }
        "maven" => { maven(results) }
        unknown => {
            warn!("Unknown format: '{}'. Will print in Maven XML format", unknown);
            maven(results)
        }
    }
}

fn version(doc: &Doc) -> &str {
    if doc.v.is_empty() {
        doc.latestVersion.as_str()
    } else {
        doc.v.as_str()
    }
}

fn maven(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    <dependency>
      <groupId>{}</groupId>
      <artifactId>{}</artifactId>
      <version>{}</version>
    </dependency>
    "#, doc.g, doc.a, version(doc))
    ).for_each(|dep| println!("{}", dep))
}

fn gradle(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    implementation '{}:{}:{}'
    "#, doc.g, doc.a, version(doc))
    ).for_each(|dep| println!("{}", dep))
}

fn gradle_kts(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    implementation("{}:{}:{}")
    "#, doc.g, doc.a, version(doc))
    ).for_each(|dep| println!("{}", dep))
}

fn sbt(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    libraryDependencies += "{}" % "{}" % "{}"
    "#, doc.g, doc.a, version(doc))
    ).for_each(|dep| println!("{}", dep))
}

fn lein(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    [{}/{} "{}"]
    "#, doc.g, doc.a, version(doc))
    ).for_each(|dep| println!("{}", dep))
}

fn ivy(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    <dependency org="{}" name="{}" rev="{}" />
    "#, doc.g, doc.a, version(doc))
    ).for_each(|dep| println!("{}", dep))
}
