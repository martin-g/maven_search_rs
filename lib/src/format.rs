use crate::types::Doc;

pub fn format(results: Vec<Doc>, output_format: &str) -> Vec<String> {
    match output_format {
        "gradle" => gradle(results),
        "gradle.kts" | "gradle_kts" | "gradlekts" => gradle_kts(results),
        "sbt" => sbt(results),
        "lein" => lein(results),
        "ivy" => ivy(results),
        "maven" => maven(results),
        unknown => {
            warn!(
                "Unknown format: '{}'. Will print in Maven XML format",
                unknown
            );
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

fn maven(results: Vec<Doc>) -> Vec<String> {
    results
        .iter()
        .map(|doc| {
            format!(
                r#"
    <dependency>
      <groupId>{}</groupId>
      <artifactId>{}</artifactId>
      <version>{}</version>
    </dependency>
    "#,
                doc.g,
                doc.a,
                version(doc)
            )
        })
        .collect()
}

fn gradle(results: Vec<Doc>) -> Vec<String> {
    results
        .iter()
        .map(|doc| {
            format!(
                r#"
    implementation '{}:{}:{}'
    "#,
                doc.g,
                doc.a,
                version(doc)
            )
        })
        .collect()
}

fn gradle_kts(results: Vec<Doc>) -> Vec<String> {
    results
        .iter()
        .map(|doc| {
            format!(
                r#"
    implementation("{}:{}:{}")
    "#,
                doc.g,
                doc.a,
                version(doc)
            )
        })
        .collect()
}

fn sbt(results: Vec<Doc>) -> Vec<String> {
    results
        .iter()
        .map(|doc| {
            format!(
                r#"
    libraryDependencies += "{}" % "{}" % "{}"
    "#,
                doc.g,
                doc.a,
                version(doc)
            )
        })
        .collect()
}

fn lein(results: Vec<Doc>) -> Vec<String> {
    results
        .iter()
        .map(|doc| {
            format!(
                r#"
    [{}/{} "{}"]
    "#,
                doc.g,
                doc.a,
                version(doc)
            )
        })
        .collect()
}

fn ivy(results: Vec<Doc>) -> Vec<String> {
    results
        .iter()
        .map(|doc| {
            format!(
                r#"
    <dependency org="{}" name="{}" rev="{}" />
    "#,
                doc.g,
                doc.a,
                version(doc)
            )
        })
        .collect()
}
