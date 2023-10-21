
use std::io::Read;
use std::mem::size_of;
use crate::io;
use crate::io::{Reader, TryReadError};

const HEADER_SIGNATURE: [u8; 4] = [0x1B, 0x4C, 0x75, 0x61];
const HEADER_VERSION: u8 = 0x53;
const HEADER_FORMAT: u8 = 0x00;
const HEADER_DATA: [u8; 6] = [0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A];
const HEADER_INSTRUCTION_SIZE: u8 = 4;

const HEADER_INTEGER_SIZE: u8 = size_of::<i64>() as u8;
const HEADER_NUMBER_SIZE: u8 = size_of::<f64>() as u8;
const HEADER_LUA_INTEGER_VALUE: i64 = 0x5678;
const HEADER_LUA_NUMBER_VALUE: f64 = 370.5;


const SIZE_OF_I32: usize = size_of::<i32>();
const SIZE_OF_I64: usize = size_of::<i64>();

const SIZE_OF_F32: usize = size_of::<f32>();
const SIZE_OF_F64: usize = size_of::<f64>();

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

impl<R: Reader> io::TryRead<R> for Header {

    fn try_read(reader: &mut R) -> Result<Self, io::TryReadError> {

        // This part of the header is of constant size
        let mut constant_buffer = [0u8; 17];

        match reader.read_exact(&mut constant_buffer) {
            Ok(()) => {},
            Err(e) => {
                return Err(TryReadError::AppError {err: "".to_string()});
            }
        }

        let signature = &constant_buffer[0..4];
        if signature != HEADER_SIGNATURE {
            return Err(TryReadError::AppError {
                err: format!("Invalid signature, expecting {:?} got {:?}", HEADER_SIGNATURE, signature) })
        }

        let version = constant_buffer[4];
        if version != HEADER_VERSION {
            return Err(TryReadError::AppError {
                err: format!("Invalid version, expecting {} got {} ", HEADER_VERSION, version)
            })
        }

        let format = constant_buffer[5];
        if format != HEADER_FORMAT {
            return  Err(TryReadError::AppError {
                err: format!("Invalid format, expecting {} got {} ", HEADER_FORMAT, format)
            })
        }

        let data = &constant_buffer[6..12];
        if data != HEADER_DATA {
            return Err(TryReadError::AppError {
                err: format!("Invalid data block, expecting {:?} got {:?}", HEADER_DATA, data)
            })
        }

        let int_size = constant_buffer[12];
        if int_size as usize != size_of::<i32>() {
            return Err(TryReadError::AppError {
                err: format!("Invalid int size, expecting {:?} got {:?}", size_of::<i32>(), int_size)
            })
        }

        let usize_size = constant_buffer[13];
        if usize_size as usize != size_of::<usize>() {
            return Err(TryReadError::AppError {
                err: format!("Invalid usize size, expecting {:?} got {:?}", size_of::<usize>(), usize_size)
            })
        }

        let instruction_size = constant_buffer[14];
        if instruction_size != HEADER_INSTRUCTION_SIZE {
            return Err(TryReadError::AppError {
                err: format!("Invalid instruction size, expecting {:?} got {:?}", HEADER_INSTRUCTION_SIZE, instruction_size)
            })
        }

        let lua_integer_size = constant_buffer[15];
        if lua_integer_size != size_of::<i32>() as u8 && lua_integer_size != size_of::<i64>() as u8 {
            return Err(TryReadError::AppError {
                err: format!("Invalid integer size, expecting one of [{}, {}] got {:?}", size_of::<i32>(), size_of::<i64>(), lua_integer_size)
            })
        }

        let lua_number_size = constant_buffer[16];
        if lua_number_size != size_of::<f32>() as u8 && lua_number_size != size_of::<f64>() as u8 {
            return Err(TryReadError::AppError {
                err: format!("Invalid number size, expecting one of [{}, {}] got {:?}", size_of::<f32>(), size_of::<f64>(), lua_number_size)
            })
        }

        // TODO: figure out endianness
        // load control integer number
        let lua_integer_value = match load_control_integer_number(reader, lua_integer_size.into()) {
            Ok(integer_value) => integer_value,
            Err(e) => return Err(e),
        };

        // load control floating point number
        let lua_number_value = match load_control_number_number(reader, lua_number_size.into()) {
            Ok(number_value) => number_value,
            Err(e) => return Err(e),
        };

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

fn load_control_integer_number(reader: &mut dyn Reader, lua_integer_size: usize) -> Result<i64, TryReadError> {

    ///////////////////////////////////////////////////////////////////////////
    // TODO: could factor out
    let mut lua_integer_value_buffer = Vec::<u8>::new();
    lua_integer_value_buffer.resize(lua_integer_size.into(), 0);

    match reader.read_exact(lua_integer_value_buffer.as_mut_slice()) {
        Ok(()) => {},
        Err(e) => {
            return Err(TryReadError::AppError { err: "".to_string()});
        }
    }
    ///////////////////////////////////////////////////////////////////////////

    let lua_integer_value = match lua_integer_size {
        SIZE_OF_I32 => {
            i32::from_le_bytes(lua_integer_value_buffer.as_slice().try_into().unwrap()) as i64
        },
        SIZE_OF_I64 => {
            i64::from_le_bytes(lua_integer_value_buffer.as_slice().try_into().unwrap())
        }
        _ => {
            return Err(TryReadError::AppError {
                err: format!("Invalid integer size, expecting one of [{}, {}] got {:?}", SIZE_OF_I32, SIZE_OF_I64, lua_integer_size)
            })
        }
    };

    return match lua_integer_value {
        HEADER_LUA_INTEGER_VALUE => Ok(lua_integer_value),
        _ => Err(TryReadError::AppError {
            err: format!("Invalid control integer value, expecting {:?} got {:?}", HEADER_LUA_INTEGER_VALUE, lua_integer_value)
        })
    };
}

fn load_control_number_number(reader: &mut dyn Reader, lua_number_size: usize) -> Result<f64, TryReadError> {

    let mut buffer = Vec::<u8>::new();
    buffer.resize(lua_number_size.into(), 0);

    match reader.read_exact(buffer.as_mut_slice()) {
        Ok(()) => {},
        Err(e) => {
            return Err(TryReadError::AppError { err: "".to_string()});
        }
    }

    let lua_number_value = match lua_number_size {
        SIZE_OF_F32 => {
            f32::from_le_bytes(buffer.as_slice().try_into().unwrap()) as f64
        },
        SIZE_OF_F64 => {
            f64::from_le_bytes(buffer.as_slice().try_into().unwrap())
        }
        _ => {
            return Err(TryReadError::AppError {
                err: format!("Invalid number size, expecting one of [{}, {}] got {:?}", SIZE_OF_F32, SIZE_OF_F64, lua_number_size)
            })
        }
    };

    return match lua_number_value {
        HEADER_LUA_NUMBER_VALUE => Ok(lua_number_value),
        _ => Err(TryReadError::AppError {
            err: format!("Invalid control number value, expecting {:?} got {:?}", HEADER_LUA_NUMBER_VALUE, lua_number_value)
        })
    };
}