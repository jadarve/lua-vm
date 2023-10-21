use std::io::Read;
use crate::io;
use crate::io::{Reader, TryReadError};

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

impl<R: Reader> io::TryRead<R> for Function {
    fn try_read(reader: &mut R) -> Result<Self, TryReadError> {

        let source = reader.read_string().unwrap();
        let line_defined = reader.read_i32().unwrap();
        let last_line_defined = reader.read_i32().unwrap();

        println!(
            "read_function: source: {} line_defined: {} : last_line_defined: {}",
            source, line_defined, last_line_defined
        );

        let num_params = reader.read_u8().unwrap();
        println!("read_function: num_params: {}", num_params);

        let is_vararg = reader.read_u8().unwrap();
        println!("read_function: is_vararg: {}", is_vararg);

        let max_stack_size = reader.read_u8().unwrap();
        println!("read_function: max_stack_size: {}", max_stack_size);

        let code = reader.read_vec_u32().unwrap();

        let function = Function {
            source,
            line_defined,
            last_line_defined,
            num_params,
            is_vararg,
            max_stack_size,
            code,
        };

        Ok(function)

        // todo!()
    }
}

