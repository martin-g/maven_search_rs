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
        "maven" | "mvn" => maven(results),
        unknown => {
            warn!("Unknown format: '{unknown}'. Will print in Maven XML format");
            maven(results)
        }
    }
}

fn version(doc: &Doc) -> &str {
    match &doc.version {
        Some(v) => v.as_str(),
        None => doc.latestVersionInfo.version.as_str(),
    }
}

fn transform<F>(results: Vec<Doc>, func: F) -> Vec<String>
where
    F: Fn(&Doc) -> String,
{
    results.iter().map(func).collect()
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
            doc.namespace,
            doc.name,
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
            doc.namespace,
            doc.name,
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
            doc.namespace,
            doc.name,
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
            doc.namespace,
            doc.name,
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
            doc.namespace,
            doc.name,
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
            doc.namespace,
            doc.name,
            version(doc)
        )
    })
}
