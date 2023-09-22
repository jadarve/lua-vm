use clap::Parser;
use lua_tools::vm53;
use std::fs;

#[derive(Parser, Debug)]
struct CliArgs {
    file: String,
}

fn main() {
    println!("lua-dump");

    let args = CliArgs::parse();

    let reader = match fs::File::open(args.file.clone()) {
        Ok(file) => file,
        Err(error) => {
            println!("Error reading file {}: {}", args.file, error);
            return;
        }
    };

    // TODO: create a 5.3 loader and start decoding.
    // TODO: document how to generate test data.
}
