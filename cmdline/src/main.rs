extern crate maven_search_lib;

mod args;

use dialoguer::{Input, Select, console::Term, theme::ColorfulTheme};
use maven_search_lib::format::format;
use maven_search_lib::http::search;
use std::error::Error;

#[macro_use]
extern crate log;

use crate::args::get_args;

static HELP: &str = r#"
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
"#;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut args = args.iter().map(String::as_str);
    let mut opts = getargs::Options::new(&mut args);
    let options = get_args(&mut opts);

    match options {
        Ok(args) => {
            if args.show_version {
                let version = env!("CARGO_PKG_VERSION");
                println!("{}", version);
                std::process::exit(0);
            }

            if args.show_help {
                println!("{}", HELP);
                std::process::exit(0);
            }

            if args.check_for_update {
                check_for_new_version();
                std::process::exit(0);
            }

            let query: String = match args.search_term {
                Some(term) => term.to_owned(),
                None => Input::<String>::new()
                    .with_prompt("Please enter the Maven query ")
                    .interact_text()?,
            };

            let output_format = match args.format {
                "" => {
                    let items = vec!["maven", "gradle", "gradlekts", "ivy", "lein", "sbt"];
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .items(&items)
                        .default(0)
                        .interact_on_opt(&Term::stderr())?;

                    match selection {
                        Some(index) => items[index],
                        None => {
                            error!(
                                "You need to select an output format. Or use '--format xyz' command line argument."
                            );
                            std::process::exit(3);
                        }
                    }
                }
                f => f,
            };

            match search(query.as_str()) {
                Ok(results) => format(results, output_format)
                    .iter()
                    .for_each(|dep| println!("{}", dep)),
                Err(err) => {
                    error!(
                        "An error occurred while searching for the latest version of '{}'\n\n{:?}",
                        query, err
                    );
                    std::process::exit(2);
                }
            }
        }
        Err(err) => {
            error!(
                "An error occurred while parsing the command line arguments: {:?}",
                err
            );
            std::process::exit(1);
        }
    }

    Ok(())
}

fn check_for_new_version() {
    use update_informer::{Check, registry};

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    let informer = update_informer::new(registry::Crates, name, version);
    if let Ok(Some(latest_version)) = informer.check_version() {
        println!(
            "A new version of this tool is available. Current {}, latest: {}. Please run 'cargo install {}' to update!",
            version, latest_version, name
        );
    }
}
