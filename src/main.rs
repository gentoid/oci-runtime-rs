extern crate reqwest;

mod commands;

fn main() {
    match commands::fetch::exec("https://github.com/coreos/etcd/releases/download/v3.3.9/etcd-v3.3.9-linux-amd64.aci") {
        Ok(res) => println!("{:?}", res),
        Err(err) => print!("{:?}", err),
    }
}
