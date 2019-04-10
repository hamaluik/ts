use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(Arg::with_name("csv-file")
            .required(true)
            .help("the .csv file to parse")
        )
        .arg(Arg::with_name("OUTPUT")
            .required(false)
            .help("the .csv file to write the results to (if not, print to STDOUT)")
        )
}
