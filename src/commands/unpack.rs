use std::io::prelude::*;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn exec(file: &str, unpack_to: &str) -> Result<String, String> {
    match File::open(file) {
        Ok(fd) => {
            let mut data: Vec<u8> = vec![];
            let mut gz = GzDecoder::new(fd);

            match gz.read_to_end(&mut data) {
                Ok(_) => {
                    match Archive::new(&data[..]).unpack(unpack_to) {
                        Ok(()) => Ok("Successfully unpacked!".to_owned()),
                        Err(err) => Err(format!("There was an error while unpack: {:?}", err)),
                    }
                },
                Err(what) => Err(format!("Cannot read gzip: {}", what))
            }
        }
        Err(e) => Err(format!("Cannot open file to read from: {}", e))
    }
}
