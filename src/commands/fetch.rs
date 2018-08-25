use reqwest;
use std::fs::File;

pub struct ContainerName<'cn> {
    name: &'cn str,
    tag: &'cn str,
}

impl<'cn> ContainerName<'cn> {
    pub fn parse(name: &str) -> Option<ContainerName> {
        let split: Vec<&str> = name.split(":").collect();
        match split.len() {
            2 => Some(ContainerName { name: split[0], tag: split[1] }),
            _ => None,
        }
    }
}

pub fn exec(url: &str, save_to: &str) -> Result<String, String> { // TODO: do not use stringified API's
    println!("{:?}", save_to);
    // ContainerName::parse(container_name).map(fetch).ok_or("Cannot parse container name")
    match reqwest::get(url) {
        Ok(mut resp) => {
            match File::create(save_to) {
                Ok(mut file) => {
                    match resp.copy_to(& mut file) {
                        Ok(bytes) => Ok(format!("Image just has been fetched and wrote {} bytes", bytes)),
                        Err(what) => Err(format!("Cannot write data: {}", what))
                    }
                }
                Err(r) => Err(format!("Cannot open file {}", r))
            }
        }
        Err(w) => Err(format!("Cannot get data {}", w))
    }
}
