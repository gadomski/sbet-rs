//! Micro-crate to read and write Smoothed Best Estimate of Trajectory (SBET) data.

#![deny(missing_docs)]

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use thiserror::Error;

const SIZE_OF_SBET_POINT_IN_BYTES: u64 = 112;

/// Crate-specific error enum.
#[derive(Debug, Error)]
pub enum Error {
    /// Extrapolation.
    #[error("extrapolation, time {time} does not fall between {start_time} and {end_time}")]
    Extrapolation {
        /// The time that does not fall between start time and end time.
        time: f64,

        /// The start time.
        start_time: f64,

        /// The end time.
        end_time: f64,
    },

    /// [std::io::Error]
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// There are no points to iterpolate.
    #[error("no points to interpolate within")]
    NoPoints,

    /// There is only one point.
    #[error("only points to interpolate within")]
    OnePoint,
}

/// Crate-specific result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Estimate the number of SBET points in a file based on file size.
///
/// # Examples
///
/// ```
/// assert_eq!(sbet::estimate_number_of_points("data/2-points.sbet").unwrap(), 2);
/// ```
pub fn estimate_number_of_points<P: AsRef<Path>>(path: P) -> Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len() / SIZE_OF_SBET_POINT_IN_BYTES)
}

/// Interpolate a sorted slice of points at a point in time.
///
/// This is pretty inefficient because it scans from the start.
///
/// TODO make this better by building an index first.
///
/// # Errors
///
/// Returns an error if:
///
/// - The time is before the first point
/// - The time is after the last point
/// - The points slice is empty or only has one point
///
/// # Examples
///
/// ```
/// use sbet::Reader;
///
/// let reader = Reader::from_path("data/2-points.sbet").unwrap();
/// let points = reader.into_iter().collect::<Result<Vec<_>, _>>().unwrap();
/// let interpolated_point = sbet::interpolate(&points, 151631.004);
/// ```
///
pub fn interpolate(points: &[Point], time: f64) -> Result<Point> {
    if points.is_empty() {
        return Err(Error::NoPoints);
    }
    if points.len() == 1 {
        return Err(Error::OnePoint);
    }
    if points[0].time > time || points.last().unwrap().time < time {
        return Err(Error::Extrapolation {
            time,
            start_time: points[0].time,
            end_time: points.last().unwrap().time,
        });
    }
    for (before, after) in points.iter().zip(points.iter().skip(1)) {
        if before.time <= time && after.time >= time {
            let factor = (time - before.time) / (after.time - before.time);
            return Ok(Point {
                time,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(missing_docs)]
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
///
/// # Examples
///
/// [Reader] implements [Iterator]:
///
/// ```
/// use sbet::Reader;
///
/// let reader = Reader::from_path("data/2-points.sbet").unwrap();
/// for result in reader {
///     let point = result.unwrap();
///     dbg!(point);
/// }
/// ```
pub struct Reader<R: Read>(pub R);

/// Use this structure to write sbet data.
pub struct Writer<W: Write>(pub W);

impl<R: Read> Reader<R> {
    /// Reads one point.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbet::Reader;
    ///
    /// let mut reader = Reader::from_path("data/2-points.sbet").unwrap();
    /// let point = reader.read_one().unwrap().unwrap();
    /// ```
    pub fn read_one(&mut self) -> Result<Option<Point>> {
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
    /// Creates a reader for the file at the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbet::Reader;
    ///
    /// let reader = Reader::from_path("data/2-points.sbet").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<BufReader<File>>> {
        File::open(path)
            .map(|f| Reader(BufReader::new(f)))
            .map_err(|e| e.into())
    }
}

impl<R: Read> Iterator for Reader<R> {
    type Item = Result<Point>;

    fn next(&mut self) -> Option<Result<Point>> {
        match self.read_one() {
            Ok(option) => option.map(Ok),
            Err(err) => Some(Err(err)),
        }
    }
}

impl<W: Write> Writer<W> {
    /// Writes one point to the writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use sbet::{Writer, Point};
    ///
    /// let mut writer = Writer(std::io::stdout());
    /// writer.write_one(Point::default());
    /// ```
    pub fn write_one(&mut self, point: Point) -> Result<()> {
        use byteorder::{LittleEndian, WriteBytesExt};
        self.0.write_f64::<LittleEndian>(point.time)?;
        self.0.write_f64::<LittleEndian>(point.latitude)?;
        self.0.write_f64::<LittleEndian>(point.longitude)?;
        self.0.write_f64::<LittleEndian>(point.altitude)?;
        self.0.write_f64::<LittleEndian>(point.x_velocity)?;
        self.0.write_f64::<LittleEndian>(point.y_velocity)?;
        self.0.write_f64::<LittleEndian>(point.z_velocity)?;
        self.0.write_f64::<LittleEndian>(point.roll)?;
        self.0.write_f64::<LittleEndian>(point.pitch)?;
        self.0.write_f64::<LittleEndian>(point.yaw)?;
        self.0.write_f64::<LittleEndian>(point.wander_angle)?;
        self.0.write_f64::<LittleEndian>(point.x_acceleration)?;
        self.0.write_f64::<LittleEndian>(point.y_acceleration)?;
        self.0.write_f64::<LittleEndian>(point.z_acceleration)?;
        self.0.write_f64::<LittleEndian>(point.x_angular_rate)?;
        self.0.write_f64::<LittleEndian>(point.y_angular_rate)?;
        self.0.write_f64::<LittleEndian>(point.z_angular_rate)?;
        Ok(())
    }
}

impl Writer<BufWriter<File>> {
    /// Creates a writer for the file at the path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sbet::Writer;
    ///
    /// let writer = Writer::from_path("outfile.sbet").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<BufWriter<File>>> {
        File::create(path)
            .map(|f| Writer(BufWriter::new(f)))
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let reader = Reader::from_path("data/2-points.sbet").unwrap();
        let points = reader.collect::<Result<Vec<Point>>>().unwrap();
        assert_eq!(2, points.len());
    }

    #[test]
    fn interpolate() {
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
        let interpolated = super::interpolate(&[first, second], 1.5).unwrap();
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
