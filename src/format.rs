use crate::types::Doc;

pub fn format(results: Vec<Doc>, fmt: &str) -> () {
    match fmt {
        "gradle" => { gradle(results) }
        "gradle.kts" | "gradle_kts" => { gradle_kts(results) }
        "sbt" => { sbt(results) }
        "lein" => { lein(results) }
        "maven" => { lein(results) }
        unknown => {
            println!("WARN: Unknown format: {}. Will print in Maven XML format", unknown);
            maven(results)
        }
    }
}

fn maven(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    <dependency>
      <groupId>{}</groupId>
      <artifactId>{}</artifactId>
      <version>{}</version>
    </dependency>
    "#, doc.g, doc.a, doc.v)
    ).for_each(|dep| println!("{}", dep))
}

fn gradle(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    implementation '{}:{}:{}'
    "#, doc.g, doc.a, doc.v)
    ).for_each(|dep| println!("{}", dep))
}

fn gradle_kts(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    implementation("{}:{}:{}")
    "#, doc.g, doc.a, doc.v)
    ).for_each(|dep| println!("{}", dep))
}

fn sbt(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    libraryDependencies += "{}" % "{}" % "{}"
    "#, doc.g, doc.a, doc.v)
    ).for_each(|dep| println!("{}", dep))
}

fn lein(results: Vec<Doc>) -> () {
    results.iter().map(|doc| format!(r#"
    [{}/{} "{}"]
    "#, doc.g, doc.a, doc.v)
    ).for_each(|dep| println!("{}", dep))
}
