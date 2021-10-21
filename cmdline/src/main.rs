extern crate maven_search_lib;

mod args;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use maven_search_lib::format::format;
use maven_search_lib::http::search;

#[macro_use]
extern crate log;

use crate::args::get_args;

static HELP: &str = r#"
maven-search [options] search-term

Search for Maven dependency

Positionals:
  search-term  The dependency you search for. E.g. "wicket-core" or "org.apache.wicket:wicket-core"                  [string]

Options:
  --version     Show version number                                                                                  [boolean]
  --format, -f  Define in which format to print dependency. (maven, gradle, gradlekts, lein, ivy, sbt)               [string] [default: "maven"]
  --help, -h    Show help                                                                                            [boolean]
"#;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args: Vec<_> = std::env::args().skip(1).collect();
    let opts = getargs::Options::new(&args);
    let options = get_args(&opts);

    match options {
        Ok(args) => {
            if args.show_version {
                let version = env!("CARGO_PKG_VERSION");
                println!("{}", version);
                std::process::exit(0);
            }

            if args.show_help {
                println!("{}", HELP);
            }

            let query: String = match args.search_term {
                Some(term) => term.to_owned(),
                None => Input::<String>::new()
                    .with_prompt("Please enter the Maven query: ")
                    .interact_text()?,
            };

            let output_format = match args.format {
                f if f.is_empty() => {
                    let items = vec!["maven", "gradle", "gradlekts", "ivy", "lein", "sbt"];
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .items(&items)
                        .default(0)
                        .interact_on_opt(&Term::stderr())?;

                    match selection {
                        Some(index) => items[index],
                        None => panic!("User did not select anything"),
                    }
                }
                f => f,
            };

            match search(query.as_str()) {
                Ok(results) => format(results, output_format),
                Err(err) => {
                    error!("{:?}", err)
                }
            }
        }
        Err(err) => {
            panic!(
                "An error occurred while parsing the command line arguments: {:?}",
                err
            );
        }
    }

    Ok(())
}
