//! Write the sbet file to a csv.

use anyhow::Error;
use clap::{App, Arg};
use sbet::Reader;
use std::{fs::File, io::Write};

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
            Arg::with_name("decimate")
                .help("decimate the data")
                .takes_value(true)
                .long("decimate")
                .short("d"),
        )
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let reader = Reader::from_path(input)?;
    let step_by = matches.value_of("decimate").unwrap().parse().unwrap();
    let mut output = File::create(matches.value_of("OUTPUT").unwrap()).unwrap();
    writeln!(output, "latitude,longitude,altitude,roll,pitch,yaw").unwrap();
    for result in reader.step_by(step_by) {
        let point = result?;
        writeln!(
            output,
            "{},{},{},{},{},{}",
            point.latitude.to_degrees(),
            point.longitude.to_degrees(),
            point.altitude,
            point.roll.to_degrees(),
            point.pitch.to_degrees(),
            point.yaw.to_degrees()
        )
        .unwrap();
    }
    Ok(())
}
