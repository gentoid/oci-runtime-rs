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

    let mut data = Vec::new();

    match header {
        [0x1f, 0x8b, 0x08, _, _, _] => {
            GzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read GZip data: {:?}", err))?;
        }
        [0x42, 0x5a, 0x68, _, _, _] => {
            BzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read BZip data: {:?}", err))?;
        }
        [0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00] => {
            XzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read XZ data: {:?}", err))?;
        }
        _ => {
            file.read_to_end(&mut data).map_err(|err| format!("Cannot read file: {:?}", err))?;
        }
    }

    Archive::new(&data[..]).unpack(unpack_to).map_err(|err| format!("There was an error while unpack tar'ed image: {:?}", err))?;

    Ok(format!("{:?} is successfully unpacked!", filename))
}
