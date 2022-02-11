use crate::types::Doc;

pub fn format(results: Vec<Doc>, output_format: &str) -> Vec<String> {
    debug!(
        "Going to format the following results with format '{:?}':\n{:?}",
        output_format, &results
    );

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

fn transform<F>(results: Vec<Doc>, func: F) -> Vec<String>
where
    F: Fn(&Doc) -> String,
{
    results.iter().map(|doc| func(doc)).collect()
}

fn maven(results: Vec<Doc>) -> Vec<String> {
    transform(results, |doc| {
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
}

fn gradle(results: Vec<Doc>) -> Vec<String> {
    transform(results, |doc| {
        format!(
            r#"
    implementation '{}:{}:{}'
    "#,
            doc.g,
            doc.a,
            version(doc)
        )
    })
}

fn gradle_kts(results: Vec<Doc>) -> Vec<String> {
    transform(results, |doc| {
        format!(
            r#"
    implementation("{}:{}:{}")
    "#,
            doc.g,
            doc.a,
            version(doc)
        )
    })
}

fn sbt(results: Vec<Doc>) -> Vec<String> {
    transform(results, |doc| {
        format!(
            r#"
    libraryDependencies += "{}" % "{}" % "{}"
    "#,
            doc.g,
            doc.a,
            version(doc)
        )
    })
}

fn lein(results: Vec<Doc>) -> Vec<String> {
    transform(results, |doc| {
        format!(
            r#"
    [{}/{} "{}"]
    "#,
            doc.g,
            doc.a,
            version(doc)
        )
    })
}

fn ivy(results: Vec<Doc>) -> Vec<String> {
    transform(results, |doc| {
        format!(
            r#"
    <dependency org="{}" name="{}" rev="{}" />
    "#,
            doc.g,
            doc.a,
            version(doc)
        )
    })
}
