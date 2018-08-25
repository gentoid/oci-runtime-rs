use std::io::prelude::*;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

use errors::*;

pub fn exec(filename: &str, unpack_to: &str) -> Result<String> {
    let file = File::open(filename).map_err(|err| format!("Cannot open file to read from: {:?}", err))?;
    let mut data: Vec<u8> = vec![];

    GzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read gzipped data: {:?}", err))?;
    Archive::new(&data[..]).unpack(unpack_to).map_err(|err| format!("There was an error while unpack tar'ed image: {:?}", err))?;

    Ok(format!("{:?} is successfully unpacked!", filename))
}
