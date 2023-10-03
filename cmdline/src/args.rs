extern crate maven_search_lib;

use getargs::{Arg, Options};

use maven_search_lib::types::{MavenResult, MavenSearchArgs};

pub fn get_args<'a, I: Iterator<Item = &'a str>>(
    opts: &'a mut Options<&'a str, I>,
) -> MavenResult<'a, MavenSearchArgs<'a>> {
    let mut res = MavenSearchArgs::default();

    while let Some(opt) = opts.next_arg().expect("argument parsing error") {
        match opt {
            Arg::Long("version") => res.show_version = true,
            Arg::Short('u') | Arg::Long("check-for-update") => res.check_for_update = true,
            Arg::Short('h') | Arg::Long("help") => res.show_help = true,
            Arg::Short('f') | Arg::Long("format") => {
                res.format = opts.value().expect("format argument parsing error")
            }
            Arg::Positional(search_term) => res.search_term = Some(search_term),
            arg => panic!("Unknown option: {arg}"),
        }
    }

    Ok(res)
}
