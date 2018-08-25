use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

use errors::*;

pub fn exec(filename: &str, unpack_to: &str) -> Result<String> {
    let file = File::open(filename).map_err(|err| format!("Cannot open file to read from: {:?}", err))?;
    let mut header = [0; 5];

    let mut handle = file.take(5);
    handle.read(&mut header).map_err(|err| format!("Cannot open read file {}: {:?}", filename, err))?;

    let mut file = handle.get_ref();
    file.seek(SeekFrom::Start(0)).map_err(|err| format!("Cannot reset cursor for {}: {:?}", filename, err))?;

    match header {
        [0x1f, 0x8b, 0x08, _, _] => {
            println!("We've found a GZip archive");
            let mut data = Vec::new();
            GzDecoder::new(file).read_to_end(&mut data).map_err(|err| format!("Cannot read gzipped data: {:?}", err))?;
            Archive::new(&data[..]).unpack(unpack_to).map_err(|err| format!("There was an error while unpack tar'ed image: {:?}", err))?;
        }
        _ => return Err(format!("We cannot recognize file format for {}", filename).into())
    }

    Ok(format!("{:?} is successfully unpacked!", filename))
}
