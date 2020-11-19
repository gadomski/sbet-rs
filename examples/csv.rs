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
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let reader = Reader::from_path(input)?;
    let mut output = File::create(matches.value_of("OUTPUT").unwrap()).unwrap();
    writeln!(output, "latitude,longitude,altitude,roll,pitch,yaw").unwrap();
    for result in reader {
        let point = result?;
        writeln!(
            output,
            "{},{},{},{},{},{}",
            point.latitude, point.longitude, point.altitude, point.roll, point.pitch, point.yaw
        )
        .unwrap();
    }
    Ok(())
}
