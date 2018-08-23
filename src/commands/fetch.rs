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

pub fn exec(container_name: &str) -> Result<&str, &str> { // TODO: do not use stringified API's
    ContainerName::parse(container_name).map(fetch).ok_or("Cannot parse container name")
}

fn fetch(cn: ContainerName) -> &str {
    "hello"
}
