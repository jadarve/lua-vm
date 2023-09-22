use std::io::Read;

pub enum TryReadError {
    IOError{err: std::io::Error},
    AppError{err: String},
}

pub trait TryRead<R: Read>: Sized {
    fn try_read(reader: &mut R) -> Result<Self, TryReadError>;
}
