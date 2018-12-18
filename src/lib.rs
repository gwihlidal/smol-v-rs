#[macro_use]
extern crate lazy_static;

use std::error::Error as StdError;
use std::io::{Read, Write};

mod ops;

/// The result of an encode or decode operation.
pub type Result<T> = ::std::result::Result<T, Error>;

/// An error that can be produced during an encode or decode operation.
pub type Error = Box<ErrorKind>;

/// The kind of error that can be produced during an encode or decode operation.
#[derive(Debug)]
pub enum ErrorKind {
    /// If the error stems from the reader/writer that is being used
    /// during encode or decode, that error will be stored and returned here.
    Io(std::io::Error),

    /// A generic error message.
    Other(String),
}

impl StdError for ErrorKind {
    fn description(&self) -> &str {
        match *self {
            ErrorKind::Io(ref err) => StdError::description(err),
            ErrorKind::Other(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::Other(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ErrorKind::Io(ref ioerr) => write!(fmt, "io error: {}", ioerr),
            ErrorKind::Other(ref s) => s.fmt(fmt),
        }
    }
}

// TODO: Bitflags
pub enum EncodeFlags {
    None,

    /// Strip all optional SPIR-V instructions (debug names etc.)
    StripDebugInfo,
}

/// Encode SPIR-V into SMOL-V.
pub fn encode(_input: &[u8], _flags: EncodeFlags) -> Result<Vec<u8>> {
    Ok(Vec::new())
}

/// Decode SMOL-V into SPIR-V.
pub fn decode(_input: &[u8]) -> Result<Vec<u8>> {
    Ok(Vec::new())
}

// Encode SPIR-V into SMOL-V.
pub fn encode_into<W>(_writer: W, _input: &[u8], _flags: EncodeFlags) -> Result<()>
where
    W: Write,
{
    Ok(())
}

/// Decode SMOL-V into SPIR-V.
pub fn decode_from<R>(_reader: R, _input: &[u8]) -> Result<()>
where
    R: Read,
{
    Ok(())
}

/// Given a SMOL-V program, get size of the decoded SPIR-V program.
/// This is the buffer size that Decode expects.
pub fn get_decoded_buffer_size(_smolv: &[u8]) -> Result<usize> {
    Ok(0)
}

#[inline(always)]
pub fn write_4<W>(mut writer: W, value: u32) -> Result<()> where W: std::io::Write
{
    writer.write(&[(value & 0xFF) as u8, ((value >> 8) & 0xFF) as u8, ((value >> 16) & 0xFF) as u8, (value >> 24) as u8])?;
    Ok(())
}