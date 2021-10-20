// use dialoguer::{theme::ColorfulTheme, Confirm};

mod args;
mod http;
mod types;
mod format;

#[macro_use]
extern crate log;

use crate::args::get_args;
use crate::format::format;
use crate::http::search;
use crate::types::MavenCoordinate;

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

fn main() {
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

            if args.show_help || args.search_term.is_none() {
                println!("{}", HELP);
            }

            if let Some(query) = args.search_term {
                let coordinate = MavenCoordinate::new(query.clone());

                match search(&coordinate) {
                    Ok(results) => {
                        format(results, args.format)
                    }
                    Err(err) => {
                        error!("{:?}", err)
                    }
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

    /*
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really want to continue?")
        .default(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really want to continue?")
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really really want to continue?")
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really really really want to continue?")
        .interact_opt()
        .unwrap()
    {
        Some(true) => println!("Looks like you want to continue"),
        Some(false) => println!("nevermind then :("),
        None => println!("Ok, we can start over later"),
    }

    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really really really really want to continue?")
        .default(true)
        .wait_for_newline(true)
        .interact_opt()
        .unwrap()
    {
        Some(true) => println!("Looks like you want to continue"),
        Some(false) => println!("nevermind then :("),
        None => println!("Ok, we can start over later"),
    } */
}
