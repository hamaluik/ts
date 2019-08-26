use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("csv-file")
                .required(true)
                .help("the .csv file to parse"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .required(false)
                .help("the .csv file to write the results to (if not, print to STDOUT)"),
        )
}
