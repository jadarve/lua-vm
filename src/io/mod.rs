use std::io::Read;

pub enum TryReadError {
    IOError { err: std::io::Error },
    AppError { err: String },
}


pub trait Reader {
    fn read_i32(&mut self) -> Result<i32, ()>;
    fn read_size_t(&mut self) -> Result<usize, ()>;
    fn read_string(&mut self) -> Result<String, ()>;
    fn read_u8(&mut self) -> Result<u8, ()>;

    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), ()>;
    fn read_vec_u32(&mut self) -> Result<Vec<u32>, ()>;
}

pub trait TryRead<R: Reader>: Sized {
    fn try_read(reader: &mut R) -> Result<Self, TryReadError>;
}
