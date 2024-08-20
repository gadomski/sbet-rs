use clap::{Parser, Subcommand};
use sbet::{Reader, Writer};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Convert an SBET file to its CSV representation.
    ToCsv {
        /// The input file path.
        ///
        /// Omit or use `-` to read from stdin.
        infile: Option<String>,

        /// The output file path.
        ///
        /// Omit or use `-` to print to stdout.
        outfile: Option<String>,

        /// Decimate the data by this amount.
        #[arg(short, long, default_value = "1")]
        decimate: usize,

        /// Include time in the output.
        #[arg(short, long)]
        include_time: bool,
    },

    /// Filter an SBET file by a start and end time.
    Filter {
        /// The input file path.
        ///
        /// Omit or use `-` to read from stdin.
        infile: Option<String>,

        /// The output file path.
        ///
        /// Omit or use `-` to print to stdout.
        outfile: Option<String>,

        /// The start time.
        #[arg(long, default_value = "-inf")]
        start_time: f64,

        /// The stop time.
        #[arg(long, default_value = "+inf")]
        stop_time: f64,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Filter {
            infile,
            outfile,
            start_time,
            stop_time,
        } => {
            let reader: Reader<Box<dyn Read>> = if let Some(infile) = infile.filter(|s| s != "-") {
                let reader = BufReader::new(File::open(infile).unwrap());
                Reader(Box::new(reader))
            } else {
                Reader(Box::new(std::io::stdin()))
            };
            let mut writer: Writer<Box<dyn Write>> =
                if let Some(outfile) = outfile.filter(|s| s != "-") {
                    let writer = BufWriter::new(File::create(outfile).unwrap());
                    Writer(Box::new(writer))
                } else {
                    Writer(Box::new(std::io::stdout()))
                };
            for result in reader {
                let point = result.unwrap();
                if (point.time >= start_time) & (point.time <= stop_time) {
                    writer.write_one(point).unwrap()
                }
            }
        }
        Command::ToCsv {
            infile,
            outfile,
            decimate,
            include_time,
        } => {
            let reader: Reader<Box<dyn Read>> = if let Some(infile) = infile.filter(|s| s != "-") {
                let reader = BufReader::new(File::open(infile).unwrap());
                Reader(Box::new(reader))
            } else {
                Reader(Box::new(std::io::stdin()))
            };
            let mut writer: Box<dyn Write> = if let Some(outfile) = outfile.filter(|s| s != "-") {
                let writer = BufWriter::new(File::create(outfile).unwrap());
                Box::new(writer)
            } else {
                Box::new(std::io::stdout())
            };
            write!(writer, "latitude,longitude,altitude").unwrap();
            if include_time {
                write!(writer, ",time").unwrap();
            }
            writeln!(writer, "").unwrap();
            for result in reader.step_by(decimate) {
                let point = result.unwrap();
                write!(
                    writer,
                    "{},{},{}",
                    point.latitude.to_degrees(),
                    point.longitude.to_degrees(),
                    point.altitude
                )
                .unwrap();
                if include_time {
                    write!(writer, ",{}", point.time).unwrap();
                }
                writeln!(writer, "").unwrap();
            }
        }
    }
}
