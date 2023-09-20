#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub signature: [u8; 4],

    // could have a version for rust impl
    pub version: u8,
    pub format: u8,
    pub data: [u8; 6],
    pub instruction_size: u8,
    pub int_size: u8,
    pub number_size: u8,

    // these can be used to determine endianness
    pub int_value: i64,
    pub number_value: f64,
}

// impl Header {
//     /// Create a new Header from a byte slice
//     pub fn from_byte_slice(bytes: &[u8]) -> Header {
//         let header = Self {
//             signature: [bytes[0], bytes[1], bytes[2], bytes[3]],
//             version: bytes[4],
//             format: bytes[5],
//             data: [
//                 bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12],
//             ],
//             instruction_size: bytes[13],
//             int_size: bytes[14],
//             number_size: bytes[15],
//             int_value: i64::from_le_bytes(bytes[16..24].try_into().unwrap()),
//             number_value: f64::from_le_bytes(bytes[24..32].try_into().unwrap()),
//         };

//         // TODO: all sorts of validations...

//         header
//     }
// }

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
