#[macro_use]
extern crate anyhow;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use anyhow::Error;

pub const SIZE_OF_SBET_POINT_IN_BYTES: u64 = 112;

pub fn interpolate(points: &[Point], time: f64) -> Result<Point, Error> {
    if points.is_empty() {
        return Err(anyhow!("no points"));
    }
    if points.len() == 1 {
        return Err(anyhow!("only one point"));
    }
    if points[0].time > time {
        return Err(anyhow!(
            "extrapolation: {} is before first point time of {}",
            time,
            points[0].time
        ));
    }
    if points.last().unwrap().time < time {
        return Err(anyhow!(
            "extrapolation: {} is after last point time of {}",
            time,
            points.last().unwrap().time
        ));
    }
    for (before, after) in points.iter().zip(points.iter().skip(1)) {
        if before.time <= time && after.time >= time {
            let factor = (time - before.time) / (after.time - before.time);
            return Ok(Point {
                time: time,
                latitude: before.latitude + factor * (after.latitude - before.latitude),
                longitude: before.longitude + factor * (after.longitude - before.longitude),
                altitude: before.altitude + factor * (after.altitude - before.altitude),
                x_velocity: before.x_velocity + factor * (after.x_velocity - before.x_velocity),
                y_velocity: before.y_velocity + factor * (after.y_velocity - before.y_velocity),
                z_velocity: before.z_velocity + factor * (after.z_velocity - before.z_velocity),
                roll: before.roll + factor * (after.roll - before.roll),
                pitch: before.pitch + factor * (after.pitch - before.pitch),
                yaw: before.yaw + factor * (after.yaw - before.yaw),
                wander_angle: before.wander_angle
                    + factor * (after.wander_angle - before.wander_angle),
                x_acceleration: before.x_acceleration
                    + factor * (after.x_acceleration - before.x_acceleration),
                y_acceleration: before.y_acceleration
                    + factor * (after.y_acceleration - before.y_acceleration),
                z_acceleration: before.z_acceleration
                    + factor * (after.z_acceleration - before.z_acceleration),
                x_angular_rate: before.x_angular_rate
                    + factor * (after.x_angular_rate - before.x_angular_rate),
                y_angular_rate: before.y_angular_rate
                    + factor * (after.y_angular_rate - before.y_angular_rate),
                z_angular_rate: before.z_angular_rate
                    + factor * (after.z_angular_rate - before.z_angular_rate),
            });
        }
    }
    unreachable!()
}

/// Smoothed Best Estimate of Trajectory (SBET) point.
#[derive(Debug, Default, PartialEq)]
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

    #[test]
    fn interpolate() -> Result<(), Error> {
        let first = Point {
            time: 1.,
            latitude: 1.,
            longitude: 1.,
            altitude: 1.,
            x_velocity: 1.,
            y_velocity: 1.,
            z_velocity: 1.,
            roll: 1.,
            pitch: 1.,
            yaw: 1.,
            wander_angle: 1.,
            x_acceleration: 1.,
            y_acceleration: 1.,
            z_acceleration: 1.,
            x_angular_rate: 1.,
            y_angular_rate: 1.,
            z_angular_rate: 1.,
        };
        let second = Point {
            time: 2.,
            latitude: 2.,
            longitude: 2.,
            altitude: 2.,
            x_velocity: 2.,
            y_velocity: 2.,
            z_velocity: 2.,
            roll: 2.,
            pitch: 2.,
            yaw: 2.,
            wander_angle: 2.,
            x_acceleration: 2.,
            y_acceleration: 2.,
            z_acceleration: 2.,
            x_angular_rate: 2.,
            y_angular_rate: 2.,
            z_angular_rate: 2.,
        };
        let interpolated = super::interpolate(&[first, second], 1.5)?;
        assert_eq!(
            interpolated,
            Point {
                time: 1.5,
                latitude: 1.5,
                longitude: 1.5,
                altitude: 1.5,
                x_velocity: 1.5,
                y_velocity: 1.5,
                z_velocity: 1.5,
                roll: 1.5,
                pitch: 1.5,
                yaw: 1.5,
                wander_angle: 1.5,
                x_acceleration: 1.5,
                y_acceleration: 1.5,
                z_acceleration: 1.5,
                x_angular_rate: 1.5,
                y_angular_rate: 1.5,
                z_angular_rate: 1.5,
            }
        );
        Ok(())
    }

    #[test]
    fn interpolate_errors() {
        assert!(super::interpolate(&[], 0.).is_err());
        assert!(super::interpolate(&[Default::default()], 0.).is_err());
        let first = Point {
            time: 1.0,
            ..Default::default()
        };
        let second = Point {
            time: 2.0,
            ..Default::default()
        };
        let points = [first, second];
        assert!(super::interpolate(&points, 0.).is_err());
        assert!(super::interpolate(&points, 0.9).is_err());
        assert!(super::interpolate(&points, 1.).is_ok());
        assert!(super::interpolate(&points, 2.).is_ok());
        assert!(super::interpolate(&points, 2.1).is_err());
    }
}
