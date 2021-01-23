use clap::{crate_authors, crate_description, crate_version, App, Arg};

pub fn build_cli() -> App<'static> {
    App::new("tm")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("SESSION")
                .about("Sets the input file to use")
                .required(true)
                .index(1)
        )
}

