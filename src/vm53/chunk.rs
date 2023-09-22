use std::convert::Into;
use std::io::Read;
use std::mem::size_of;
use crate::io;
use crate::io::TryReadError;

const HEADER_SIGNATURE: [u8; 4] = [0x1B, 0x4C, 0x75, 0x61];
const HEADER_VERSION: u8 = 0x53;
const HEADER_FORMAT: u8 = 0x00;
const HEADER_DATA: [u8; 6] = [0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A];
const HEADER_INSTRUCTION_SIZE: u8 = 4;

const HEADER_INTEGER_SIZE: u8 = size_of::<i64>() as u8;
const HEADER_NUMBER_SIZE: u8 = size_of::<f64>() as u8;
const HEADER_LUA_INTEGER_VALUE: i64 = 0x5678;
const HEADER_LUA_NUMBER_VALUE: f64 = 370.5;

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub signature: [u8; 4],

    // could have a version for rust impl
    pub version: u8,
    pub format: u8,
    pub data: [u8; 6],
    pub int_size: u8,
    pub usize_size: u8,
    pub instruction_size: u8,
    pub lua_integer_size: u8,
    pub lua_number_size: u8,
    // these can be used to determine endianness
    pub int_value: i64,
    pub number_value: f64,
}

impl<R: Read> io::TryRead<R> for Header {

    fn try_read(reader: &mut R) -> Result<Self, io::TryReadError> {

        // TODO: could take the size of the Header from the header itself
        let mut bytes = [0u8; 34];

        match reader.read_exact(&mut bytes) {
            Ok(()) => {},
            Err(e) => {
                return Err(TryReadError::IOError { err: e});
            }
        }

        let signature = &bytes[0..4];
        if signature != HEADER_SIGNATURE {
            return Err(TryReadError::AppError {
                err: format!("Invalid signature, expecting {:?} got {:?}", HEADER_SIGNATURE, signature) })
        }

        let version = bytes[4];
        if version != HEADER_VERSION {
            return Err(TryReadError::AppError {
                err: format!("Invalid version, expecting {} got {} ", HEADER_VERSION, version)
            })
        }

        let format = bytes[5];
        if format != HEADER_FORMAT {
            return  Err(TryReadError::AppError {
                err: format!("Invalid format, expecting {} got {} ", HEADER_FORMAT, format)
            })
        }

        let data = &bytes[6..12];
        if data != HEADER_DATA {
            return Err(TryReadError::AppError {
                err: format!("Invalid data block, expecting {:?} got {:?}", HEADER_DATA, data)
            })
        }

        let int_size = bytes[12];
        if int_size as usize != size_of::<i32>() {
            return Err(TryReadError::AppError {
                err: format!("Invalid int size, expecting {:?} got {:?}", size_of::<i32>(), int_size)
            })
        }

        let usize_size = bytes[13];
        if usize_size as usize != size_of::<usize>() {
            return Err(TryReadError::AppError {
                err: format!("Invalid usize size, expecting {:?} got {:?}", size_of::<usize>(), usize_size)
            })
        }

        let instruction_size = bytes[14];
        if instruction_size != HEADER_INSTRUCTION_SIZE {
            return Err(TryReadError::AppError {
                err: format!("Invalid instruction size, expecting {:?} got {:?}", HEADER_INSTRUCTION_SIZE, instruction_size)
            })
        }

        let lua_integer_size = bytes[15];
        if lua_integer_size != HEADER_INTEGER_SIZE {
            return Err(TryReadError::AppError {
                err: format!("Invalid integer size, expecting {:?} got {:?}", HEADER_INTEGER_SIZE, lua_integer_size)
            })
        }

        let lua_number_size = bytes[16];
        if lua_number_size != HEADER_NUMBER_SIZE {
            return Err(TryReadError::AppError {
                err: format!("Invalid number size, expecting {:?} got {:?}", HEADER_NUMBER_SIZE, lua_number_size)
            })
        }

        // TODO: should read as many bytes as lua_integer_size
        let lua_integer_value = i64::from_le_bytes(bytes[17..25].try_into().unwrap());
        if lua_integer_value != HEADER_LUA_INTEGER_VALUE {
            return Err(TryReadError::AppError {
                err: format!("Invalid control integer value, expecting {:?} got {:?}", HEADER_LUA_INTEGER_VALUE, lua_integer_value)
            })
        }

        // TODO: should read as many bytes as lua_number_size
        let lua_number_value = f64::from_le_bytes(bytes[25..33].try_into().unwrap());
        if lua_number_value != HEADER_LUA_NUMBER_VALUE {
            return Err(TryReadError::AppError {
                err: format!("Invalid control number value, expecting {:?} got {:?}", HEADER_LUA_NUMBER_VALUE, lua_number_value)
            })
        }

        let header = Header {
            signature: <[u8; 4]>::try_from(signature).unwrap(),
            version,
            format,
            data: <[u8; 6]>::try_from(data).unwrap(),
            int_size,
            usize_size,
            instruction_size,
            lua_integer_size,
            lua_number_size,
            int_value: lua_integer_value,
            number_value: lua_number_value,
        };

        Ok(header)
    }
}


#[derive(Debug, Clone)]
pub struct Function {
    pub source: String,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
}
