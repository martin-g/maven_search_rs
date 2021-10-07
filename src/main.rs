use std::process;
use getargs::{Error, Opt, Options, Result as GetArgsResult};
use serde_json::{Result as JsonResult, Value};
use serde::{Deserialize};
// use dialoguer::{theme::ColorfulTheme, Confirm};

static HELP: &str = r#"
maven-search [args] <query-string>

search for Maven dependency

Positionals:
  query-string  the dependency you search for                                                                        [string]

Options:
  --version     Show version number                                                                                  [boolean]
  --format, -f  Define in which format to print dependency. (gradle, gradlekts, gradlegroovy, maven, sbt)            [string] [default: "maven"]
  --help, -h    Show help                                                                                            [boolean]
        "#;

#[derive(Default, Debug)]
struct MavenSearchArgs<'a> {
    show_version: bool,
    show_help: bool,
    format: &'a str ,
    search_term: Option<&'a  String>,
}

fn parse_args<'a>(opts: &'a Options<'a, String>) -> GetArgsResult<MavenSearchArgs<'a>> {
    let mut res = MavenSearchArgs::default();
    while let Some(opt) = opts.next() {
        match opt? {
            Opt::Short('h') | Opt::Long("help") => res.show_help = true,
            Opt::Long("version") => res.show_version = true,
            Opt::Short('f') | Opt::Long("format") => res.format = opts.value_str()?,
            opt => return Err(Error::UnknownOpt(opt)),
        }
    }
    res.search_term = opts.args().first();
    Ok(res)
}

#[derive(Debug)]
struct MavenCoordinate {
    group_id: String,
    artifact_id: String,
    version: String,
}

impl MavenCoordinate {
    fn new(artifact_id: String) -> MavenCoordinate {
      MavenCoordinate {
          group_id: "".to_string(),
          artifact_id: artifact_id,
          version: "".to_string(),
        }
    }
}


#[derive(Debug)]
struct SearchResult {
    coordinates: Vec<MavenCoordinate>
}


#[derive(Debug, Deserialize)]
struct Doc {
    id: String,
    g: String,
    a: String,
    v: String,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    docs: Vec<Doc>
}


#[derive(Debug, Deserialize)]
struct HttpResponse {
    response: SearchResponse
}

fn search<'a>(coordinate: &'a MavenCoordinate) -> Result<SearchResult, reqwest::Error> {

    let url = format!("https://search.maven.org/solrsearch/select?rows=2&q=a:{}&core=gav", 
        coordinate.artifact_id);

    let resp = reqwest::blocking::get(url)?
        .text()?;
    
    match get_docs(resp) {
        Ok(json) => {println!("RESPONSE: {:#?}", json);},
        Err(err) =>  {println!("ERROR: {:#?}", err);}
    }

    Ok(SearchResult {coordinates: Vec::new()})

}

fn get_docs(response: String) -> JsonResult<Vec<Doc>> {

    let v: HttpResponse = serde_json::from_str(response.as_str())?;

    Ok(v.response.docs)
}

fn main() {

    let args: Vec<_> = std::env::args().skip(1).collect();
    let opts = Options::new(&args);
    let options = match parse_args(&opts) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("usage error: {}", e);
            process::exit(1);
        }
    };
    println!("{:#?}", options);

    if options.show_version {
        println!("0.1.0");
    }

    if options.show_help || options.search_term.is_none() {
        println!("{}", HELP);
    }

    if let Some(query) = options.search_term {
        let coordinate = MavenCoordinate::new(query.clone());
        let results = search(&coordinate);
        println!("Results: {:?}", results)
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