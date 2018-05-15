use clap::{Arg, App, ArgMatches};


pub fn match_arguments() -> ArgMatches<'static> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))

        // flags
        .arg(Arg::with_name("edit")
            .short("e")
            .long("edit")
            .help("Edit bookmark file"))

        // arguments
        .arg(Arg::with_name("KEY")
            .help("Key used to get a website")
            .index(1))
        .get_matches();
    
    matches
}
