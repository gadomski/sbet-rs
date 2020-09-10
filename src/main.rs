use anyhow::Error;
use clap::{App, Arg, SubCommand};
use sbet::Reader;

fn main() -> Result<(), Error> {
    let matches = App::new("sbet")
        .subcommand(
            SubCommand::with_name("to-csv")
                .about("converts sbet to csv")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("decimate")
                        .short("d")
                        .long("decimate")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("to-csv") {
        let input = matches.value_of("INPUT").unwrap();
        let decimate = matches
            .value_of("decimate")
            .unwrap_or("1")
            .parse::<usize>()
            .unwrap();
        let reader = Reader::from_path(input)?;
        println!("time,latitude,longitude,roll,pitch,yaw");
        for point in reader.step_by(decimate) {
            let point = point?;
            println!(
                "{},{},{},{},{},{}",
                point.time,
                point.latitude.to_degrees(),
                point.longitude.to_degrees(),
                point.roll,
                point.pitch,
                point.yaw
            );
        }
    }
    Ok(())
}
