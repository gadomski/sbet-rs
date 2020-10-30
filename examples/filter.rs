//! Filter the sbet file based on start and/or end time.

use anyhow::Error;
use clap::{App, Arg};
use sbet::{Reader, Writer};

fn main() -> Result<(), Error> {
    let matches = App::new("sbet")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("start-time")
                .long("start-time")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stop-time")
                .long("stop-time")
                .takes_value(true),
        )
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let reader = Reader::from_path(input)?;
    let output = matches.value_of("OUTPUT").unwrap();
    let mut writer = Writer::from_path(output)?;
    let start_time: f64 = matches.value_of("start-time").unwrap_or("-inf").parse()?;
    let stop_time: f64 = matches.value_of("stop-time").unwrap_or("inf").parse()?;
    for result in reader {
        let point = result?;
        if (point.time >= start_time) & (point.time <= stop_time) {
            writer.write_one(point)?;
        }
    }
    Ok(())
}
