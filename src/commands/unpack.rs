use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;

use errors::*;

pub fn exec(filename: &str, unpack_to: &str) -> Result<String> {
    let file = File::open(filename).map_err(|err| format!("Cannot open file to read from: {:?}", err))?;
    let mut header = [0; 6];

    let mut handle = file.take(6);
    handle.read(&mut header).map_err(|err| format!("Cannot open read file {}: {:?}", filename, err))?;

    let mut file = handle.get_ref();
    file.seek(SeekFrom::Start(0)).map_err(|err| format!("Cannot reset cursor for {}: {:?}", filename, err))?;

    match header {
        [0x1f, 0x8b, 0x08, _, _, _] => {
            println!("We've found a GZip archive");
            let mut data = Vec::new();
            GzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read gzipped data: {:?}", err))?;
            Archive::new(&data[..]).unpack(unpack_to).map_err(|err| format!("There was an error while unpack tar'ed image: {:?}", err))?;
        }
        [0x42, 0x5a, 0x68, _, _, _] => {
            println!("We've found a BZip2 archive");
            let mut data = Vec::new();
            BzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read bzipped data: {:?}", err))?;
            Archive::new(&data[..]).unpack(unpack_to).map_err(|err| format!("There was an error while unpack tar'ed image: {:?}", err))?;
        }
        [0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00] => {
            println!("We've found an XZ archive");
            let mut data = Vec::new();
            XzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read bzipped data: {:?}", err))?;
            Archive::new(&data[..]).unpack(unpack_to).map_err(|err| format!("There was an error while unpack tar'ed image: {:?}", err))?;
        }
        _ => return Err(format!("We cannot recognize file format for {}", filename).into())
    }

    Ok(format!("{:?} is successfully unpacked!", filename))
}
