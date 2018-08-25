use std::io::prelude::*;
use std::fs::File;
use flate2::read::GzDecoder;

pub fn exec(file: &str, unpack_to: &str) -> Result<String, String> {
    match File::open(file) {
        Ok(fd) => {
            // let data = fd.bytes().collect();
            match File::create(unpack_to) {
                Ok(mut save_to) => {
                    println!("{:?}", fd);
                    let mut data: Vec<u8> = vec![];
                    let mut gz = GzDecoder::new(fd);
                    match gz.header() {
                        Some(hd) => println!("{:?}", hd),
                        None => return Err("Cannot read header".to_owned()),
                    }

                    match gz.read_to_end(&mut data) {
                        Ok(bytes) => {
                            println!("Read: {}", bytes);
                            match save_to.write_all(&data[..]) {
                                Ok(()) => Ok("File saved".to_owned()),
                                Err(e) => Err(format!("Cannot write file: {}", e)),
                            }
                        },
                        Err(what) => Err(format!("Cannot read gzip: {}", what))
                    }
                }
                Err(r) => Err(format!("Cannot open file to write to {}", r))
            }
        }
        Err(e) => Err(format!("Cannot open file to read from: {}", e))
    }
}
