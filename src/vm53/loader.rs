use crate::vm53::chunk::{Function, Header};
use std::{io::Read, string};
use crate::io::{TryRead, TryReadError};

// pub trait ChunkReader {
//     fn read_header(&mut self) -> Result<Header, ()>;

//     fn read_function(&mut self) -> Result<Function, ()>;

//     fn read_size_t(&mut self) -> Result<usize, ()>;

//     fn read_string(&mut self) -> Result<String, ()>;

//     fn read_u8(&mut self) -> Result<u8, ()>;

//     fn read_i32(&mut self) -> Result<i32, ()>;

//     fn read_vecu32(&mut self) -> Result<Vec<u32>, ()>;
// }

pub struct Lua53ChunkReader<R: Read> {
    pub reader: R,
}

impl<R: Read> Lua53ChunkReader<R> {
    pub fn read_header(&mut self) -> Result<Header, ()> {

        return match Header::try_read(&mut self.reader) {
            Ok(header) => Ok(header),

            // TODO
            Err(_) => Err(()),
        }
    }

    pub fn read_function(&mut self) -> Result<Function, ()> {
        let source = self.read_string().unwrap();
        let line_defined = self.read_i32().unwrap();
        let last_line_defined = self.read_i32().unwrap();

        println!(
            "read_function: source: {} line_defined: {} : last_line_defined: {}",
            source, line_defined, last_line_defined
        );

        let num_params = self.read_u8().unwrap();
        println!("read_function: num_params: {}", num_params);

        let is_vararg = self.read_u8().unwrap();
        println!("read_function: is_vararg: {}", is_vararg);

        let max_stack_size = self.read_u8().unwrap();
        println!("read_function: max_stack_size: {}", max_stack_size);

        let code = self.read_vecu32().unwrap();

        let function = Function {
            source,
            line_defined,
            last_line_defined,
            num_params,
            is_vararg,
            max_stack_size,
            code: code,
        };

        Ok(function)
    }

    pub fn read_size_t(&mut self) -> Result<usize, ()> {
        let byte_count = std::mem::size_of::<usize>();

        // let lua_byte_count = ((byte_count * 8) + 6) / 7;

        let mut size_t = 0_usize;

        let mut buffer = [0u8; 1];

        let mut i = 0;
        loop {
            self.reader.read_exact(&mut buffer[0..1]).unwrap();

            let byte = buffer[0];
            size_t <<= 7;
            size_t |= (byte & 0x7f) as usize;

            // check we have reached the end of the size_t
            if byte & 0x80u8 != 0 {
                break;
            }

            // if we read more than byte_count, the size_t is too big
            i += 1;
            if i == byte_count {
                return Err(());
            }
        }

        Ok(size_t)
    }

    pub fn read_vecu32(&mut self) -> Result<Vec<u32>, ()> {
        let size_t = self.read_size_t().unwrap();

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

    pub fn read_string(&mut self) -> Result<String, ()> {
        let size_t = self.read_size_t().unwrap() - 1;

        let buffer = &mut vec![0u8; size_t];
        self.reader.read_exact(buffer).unwrap();

        // TODO: omit the last byte in the buffer to remove the null character
        let string_value = String::from_utf8_lossy(&buffer[0..size_t]).to_string();
        println!(
            "read_string: size: {}: {:?} : {}",
            size_t, buffer, string_value
        );
        Ok(string_value)
    }

    pub fn read_u8(&mut self) -> Result<u8, ()> {
        let mut buffer = [0u8; 1];
        self.reader.read_exact(&mut buffer).unwrap();

        Ok(buffer[0])
    }

    pub fn read_i32(&mut self) -> Result<i32, ()> {
        let byte_count = std::mem::size_of::<i32>();

        // let lua_byte_count = ((byte_count * 8) + 6) / 7;

        let mut u_value = 0_u32;

        let mut buffer = [0u8; 1];

        let mut i = 0;
        loop {
            self.reader.read_exact(&mut buffer[0..1]).unwrap();

            let byte = buffer[0];
            u_value <<= 7;
            u_value |= (byte & 0x7f) as u32;

            // check we have reached the end of the u_value
            if byte & 0x80u8 != 0 {
                break;
            }

            // if we read more than byte_count, the u_value is too big
            i += 1;
            if i == byte_count {
                return Err(());
            }
        }

        Ok(i32::from_le_bytes(u_value.to_ne_bytes()))
    }
}
