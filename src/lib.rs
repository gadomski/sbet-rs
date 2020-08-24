use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use anyhow::Error;

/// Smoothed Best Estimate of Trajectory (SBET) point.
pub struct Point {
    pub time: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub x_velocity: f64,
    pub y_velocity: f64,
    pub z_velocity: f64,
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub wander_angle: f64,
    pub x_acceleration: f64,
    pub y_acceleration: f64,
    pub z_acceleration: f64,
    pub x_angular_rate: f64,
    pub y_angular_rate: f64,
    pub z_angular_rate: f64,
}

/// Use this structure to read sbet data from a source.
pub struct Reader<R: Read>(R);

impl<R: Read> Reader<R> {
    /// Reads one point from the reader.
    pub fn read_one(&mut self) -> Result<Option<Point>, Error> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::ErrorKind;
        let time = match self.0.read_f64::<LittleEndian>() {
            Ok(time) => time,
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => return Ok(None),
                _ => return Err(err.into()),
            },
        };
        Ok(Some(Point {
            time,
            latitude: self.0.read_f64::<LittleEndian>()?,
            longitude: self.0.read_f64::<LittleEndian>()?,
            altitude: self.0.read_f64::<LittleEndian>()?,
            x_velocity: self.0.read_f64::<LittleEndian>()?,
            y_velocity: self.0.read_f64::<LittleEndian>()?,
            z_velocity: self.0.read_f64::<LittleEndian>()?,
            roll: self.0.read_f64::<LittleEndian>()?,
            pitch: self.0.read_f64::<LittleEndian>()?,
            yaw: self.0.read_f64::<LittleEndian>()?,
            wander_angle: self.0.read_f64::<LittleEndian>()?,
            x_acceleration: self.0.read_f64::<LittleEndian>()?,
            y_acceleration: self.0.read_f64::<LittleEndian>()?,
            z_acceleration: self.0.read_f64::<LittleEndian>()?,
            x_angular_rate: self.0.read_f64::<LittleEndian>()?,
            y_angular_rate: self.0.read_f64::<LittleEndian>()?,
            z_angular_rate: self.0.read_f64::<LittleEndian>()?,
        }))
    }
}

impl Reader<BufReader<File>> {
    /// Create a reader for the provided file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>, Error> {
        File::open(path)
            .map(|f| Reader(BufReader::new(f)))
            .map_err(|e| e.into())
    }
}

impl<R: Read> Iterator for Reader<R> {
    type Item = Result<Point, Error>;

    fn next(&mut self) -> Option<Result<Point, Error>> {
        match self.read_one() {
            Ok(option) => match option {
                Some(point) => Some(Ok(point)),
                None => None,
            },
            Err(err) => Some(Err(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() -> Result<(), Error> {
        let reader = Reader::from_path("data/2-points.sbet")?;
        let points = reader.collect::<Vec<Result<Point, Error>>>();
        assert_eq!(2, points.len());
        Ok(())
    }
}
