# maven_search_rs
Command line application for searching in https://central.sonatype.com

# Usage

## Non-interactive

    $ maven-search -f maven wicket-core

The above will search for the latest version of a Maven artifact with id `wicket-core`.
Sample output:

```xml
<dependency>
    <groupId>org.apache.wicket</groupId>
    <artifactId>wicket-core</artifactId>
    <version>9.8.0</version>
</dependency>
```

### Help

    $ maven-search -h

prints

```
maven-search [options] query

Search for Maven dependency

Positionals:
  query  The dependency you search for. E.g. "wicket-core" or "g:org.apache.wicket AND a:wicket-core"                [string]
         The syntax is the same as at https://search.maven.org/

Options:
  --version                 Show version number and exit
  --format, -f [string]     Define in which format to print dependency. (maven, gradle, gradlekts, lein, ivy, sbt). Default: "maven"
  --check-for-update, -u    Checks whether there is a new version of this tool available and exit
  --help, -h                Show this help and exit
```

## Interactive

    $ maven-search

starts an interactive session where the user has to type the query and select the output format.

[![asciicast](https://asciinema.org/a/447171.svg)](https://asciinema.org/a/447171)
