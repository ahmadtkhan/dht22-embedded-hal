//! Has Checksum error, and timeout errors

use core::fmt;

#[derive(Debug)]
pub enum SensorError {
    ChecksumMismatch,
    TimeoutWaitingForHigh,
    TimeoutWaitingForLow,
    TimeoutGeneric,
    Other(&'static str),
}
impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorError::ChecksumMismatch => write!(f, "Checksum mismatch"),
            SensorError::TimeoutWaitingForHigh => write!(f, "Timed out waiting for pin HIGH"),
            SensorError::TimeoutWaitingForLow => write!(f, "Timed out waiting for pin LOW"),
            SensorError::TimeoutGeneric => write!(f, "General timeout occurred"),
            SensorError::Other(msg) => write!(f, "{}", msg),
        }
    }
}
