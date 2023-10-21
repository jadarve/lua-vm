use clap::Parser;
use std::fmt;
use std::fs;
use std::io::Read;
use std::mem;

use lua_tools::vm53;
use lua_tools::io::{Reader};

#[derive(Parser, Debug)]
struct CliArgs {
    file: String,
}

fn main() {
    println!("lua");

    let args = CliArgs::parse();

    // let buffer = fs::read(args.file.clone()).unwrap();

    // let chunk_header = chunk::Header::from_byte_slice(&buffer[0..32]);

    // println!("{:#?}", chunk_header);

    // let (source_size, offset) = loader::load_size_t(&buffer[32..buffer.len()]);
    // println!("source_size: {} offset: {}", source_size, offset);

    // let function_source = loader::load_raw_string(&buffer[32..buffer.len()]);
    // println!("function_source: {}", function_source);

    // let function_buffer = buffer[31..buffer.len()].to_vec();
    // println!("{:#?}\n", function_buffer);

    check_loader(&args);
}

fn check_loader(args: &CliArgs) {
    println!("check_loader");

    let mut reader = fs::File::open(args.file.clone()).unwrap();

    let mut chunk_reader = vm53::Lua53ChunkReader { reader };

    let header = chunk_reader.read_header().unwrap();
    println!("{:#?}", header);

    // ignore one byte. TODO
    let unused_byte = chunk_reader.read_u8().unwrap();
    println!("unused byte: {}", unused_byte);

    let function = chunk_reader.read_function().unwrap();
    println!("{:#?}", function);

    // for i in 0..4 {
    //     println!("next byte: {} : {}", i, chunk_reader.read_u8().unwrap());
    // }

    // let mut bytes = [0u8; 4];
    // match reader.read_exact(&mut bytes) {
    //     Ok(()) => {
    //         println!("following bytes: {:?}", bytes);
    //     },
    //     Err(_) => {},
    // }

    // let upsize = chunk_reader.read_u8().unwrap();
    // println!("upsize: {}", upsize);
    //
    // let function = chunk_reader.read_function().unwrap();
    // println!("{:#?}", function);
}
