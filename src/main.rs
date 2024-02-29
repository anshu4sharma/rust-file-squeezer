use std::io::{self, prelude::*};
use std::{env, fs::File};

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "compress" => {
            let mut file_to_compress = File::open(args[2].as_str())?;
            let mut file_to_compress_buffer = Vec::new();
            file_to_compress
                .read_to_end(&mut file_to_compress_buffer)
                .unwrap();
            let cb: Vec<u8> = flate3::deflate(&file_to_compress_buffer);
            let mut files = File::create(args[2].as_str().to_string()+".squeeze").unwrap();
            files.write_all(&cb).unwrap();
        }
        "decompress" => {
            let mut file_to_decompress = File::open(args[2].as_str())?;
            let mut buffer = Vec::new();
            file_to_decompress.read_to_end(&mut buffer)?;
            let mut file_to_decompress_file = File::create(args[2].as_str().split(".squeeze").next().unwrap()).unwrap();
            let decompressed_file_buffer: Vec<u8> = flate3::inflate(&buffer);
            file_to_decompress_file.write_all(&decompressed_file_buffer)?;
        }
        _ => {
            eprintln!("Invalid Input !")
        }
    }

    Ok(())
}

