use crate::io::{Reader, TryRead, TryReadError};
use crate::vm53::function::Function;
use crate::vm53::header::Header;
use std::{io::Read, string};

const SHORT_STRING_MAX_LENGTH: usize = 40;

pub struct Lua53ChunkReader<R: Read> {
    pub reader: R,
}

impl<R: Read> Reader for Lua53ChunkReader<R> {
    fn read_i32(&mut self) -> Result<i32, ()> {
        let byte_count = std::mem::size_of::<i32>();

        let buffer = &mut vec![0u8; byte_count];

        return match self.reader.read_exact(buffer) {
            Ok(()) => {
                Ok(i32::from_le_bytes(buffer.as_slice().try_into().unwrap()))
            }
            Err(e) => {
                Err(())
            }
        };
    }

    fn read_size_t(&mut self) -> Result<usize, ()> {
        let byte_count = std::mem::size_of::<usize>();

        let buffer = &mut vec![0u8; byte_count];

        return match self.reader.read_exact(buffer) {
            Ok(()) => {
                Ok(usize::from_le_bytes(buffer.as_slice().try_into().unwrap()))
            }
            Err(e) => {
                Err(())
            }
        };
    }

    fn read_string(&mut self) -> Result<String, ()> {
        let first_byte = match self.read_u8() {
            Ok(value) => value,
            Err(e) => {
                return Err(());
            }
        };

        let size_t = match first_byte {
            // fully load the string length
            0xFFu8 => match self.read_size_t() {
                Ok(value) => value,
                Err(e) => return Err(())
            }
            other_value => {
                other_value as usize
            }
        };

        return if size_t == 0x00_usize {
            println!("empty string");
            Ok(String::new())
        } else if size_t - 1 <= SHORT_STRING_MAX_LENGTH {

            // the null terminator is not stored as part of the buffer
            let buf_size = size_t - 1;
            println!("short string: {}", buf_size);
            // short string
            let buffer = &mut vec![0u8; buf_size];
            self.reader.read_exact(buffer).unwrap();

            let string_value = String::from_utf8_lossy(&buffer[0..buf_size]).to_string();

            println!(
                "read_string: size: {}: {:?} : {}",
                size_t, buffer, string_value
            );
            Ok(string_value)
        } else {

            // TODO: allocate string buffer in Lua stack
            // the null terminator is not stored as part of the buffer
            let buf_size = size_t - 1;
            println!("long string: {}", buf_size);
            // short string
            let buffer = &mut vec![0u8; buf_size];
            self.reader.read_exact(buffer).unwrap();

            let string_value = String::from_utf8_lossy(&buffer[0..buf_size]).to_string();

            println!(
                "read_string: size: {}: {:?} : {}",
                size_t, buffer, string_value
            );
            Ok(string_value)
        };
    }

    fn read_u8(&mut self) -> Result<u8, ()> {
        let mut buffer = [0u8; 1];
        self.reader.read_exact(&mut buffer).unwrap();

        Ok(buffer[0])
    }

    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), ()> {
        match self.reader.read_exact(buffer) {
            Ok(_) => { Ok(()) }
            Err(_) => { Err(()) }
        }
    }

    fn read_vec_u32(&mut self) -> Result<Vec<u32>, ()> {
        let size_t = self.read_i32().unwrap() as usize;

        println!("read_vecu32 size_t: {}", size_t);

        let mut vec = vec![0u32; size_t];

        let mut buffer = [0u8; std::mem::size_of::<u32>()];

        for i in 0..size_t {
            self.reader.read_exact(&mut buffer).unwrap();

            // FIXME: endianneess
            vec[i] = u32::from_le_bytes(buffer);
        }

        Ok(vec)
    }
}

impl<R: Read> Lua53ChunkReader<R> {
    pub fn read_header(&mut self) -> Result<Header, ()> {
        return match Header::try_read(self) {
            Ok(header) => Ok(header),
            Err(_) => Err(()),
        };
    }

    pub fn read_function(&mut self) -> Result<Function, ()> {
        return match Function::try_read(self) {
            Ok(function) => Ok(function),
            Err(_) => Err(()),
        };
    }
}
