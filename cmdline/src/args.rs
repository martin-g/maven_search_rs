extern crate maven_search_lib;

use getargs::{Error, Opt, Options};

use maven_search_lib::types::{MavenError::Args, MavenResult, MavenSearchArgs};

pub fn get_args<'a>(opts: &'a Options<'a, String>) -> MavenResult<'a, MavenSearchArgs<'a>> {
    let mut res = MavenSearchArgs::default();

    while let Some(opt) = opts.next() {
        match opt? {
            Opt::Long("version") => res.show_version = true,
            Opt::Short('h') | Opt::Long("help") => res.show_help = true,
            Opt::Short('f') | Opt::Long("format") => res.format = opts.value_str()?,
            opt => return Err(Args(Error::UnknownOpt(opt))),
        }
    }
    res.search_term = opts.args().first();
    Ok(res)
}
