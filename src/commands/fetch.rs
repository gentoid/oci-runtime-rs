use reqwest;
use std::fs::File;

use errors::*;

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

pub fn exec(url: &str, save_to: &str) -> Result<String> { // TODO: do not use stringified API's
    let mut file = File::create(save_to).map_err(|err| format!("Cannot open file to save image to: {}", err))?;
    let mut resp = reqwest::get(url).map_err(|err| format!("Cannot get data: {:?}", err))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!(
            "Status code isn't success: {} {:?}",
            status.as_u16(),
            status.canonical_reason().unwrap_or("Unknown reason")).into())
        }
    resp.copy_to(& mut file).map_err(|err| format!("Cannot write data: {:?}", err))?;
    Ok(format!("Image has been saved to {:?}", save_to))
}
