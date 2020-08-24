use anyhow::Error;
use clap::{App, Arg, SubCommand};
use sbet::Reader;

fn main() -> Result<(), Error> {
    let matches = App::new("sbet")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .subcommand(SubCommand::with_name("to-csv").about("converts sbet to csv"))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let reader = Reader::from_path(input)?;

    if let Some(_) = matches.subcommand_matches("to-csv") {
        for point in reader {
            let point = point?;
            println!(
                "{},{}",
                point.latitude.to_degrees(),
                point.longitude.to_degrees()
            );
        }
    }
    Ok(())
}
