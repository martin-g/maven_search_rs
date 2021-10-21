# maven_search_rs
Command line application for searching in https://search.maven.org

# Usage

## Non-interactive

    $ maven-search -f maven wicket-core

The above will search for the latest version of a Maven artifact with id `wicket-core`. 
Sample output:

```xml
<dependency>
    <groupId>org.apache.wicket</groupId>
    <artifactId>wicket-core</artifactId>
    <version>9.5.0</version>
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
--version     Show version number                                                                                  [boolean]
--format, -f  Define in which format to print dependency. (maven, gradle, gradlekts, lein, ivy, sbt)               [string] [default: "maven"]
--help, -h    Show this help                                                                                       [boolean]
```

## Interactive

    $ maven-search

starts an interactive session where the user has to type the query and select the output format.

[![asciicast](https://asciinema.org/a/CgJHxNDPycmEDG4FZCE0hwSDy.svg)](https://asciinema.org/a/CgJHxNDPycmEDG4FZCE0hwSDy)