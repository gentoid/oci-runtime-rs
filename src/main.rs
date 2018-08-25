extern crate clap;
extern crate reqwest;

mod commands;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("roci")
        .version("0.1.0")
        .author("Viktor L.")
        .about("An application to run containers")
        .subcommand(SubCommand::with_name("fetch")
            .about("Fetches images from remote registries")
            .arg(Arg::with_name("IMAGE_URI")
                .help("Image URI to fetch")
                .required(true)
                .index(1))
            .arg(Arg::with_name("SAVE_TO")
                .help("Where it should save fetched image. Default is: image.aci")
                .required(false)
                .index(2)))
        .get_matches();

    if let Some(fetch_match) = matches.subcommand_matches("fetch") {
        let uri = fetch_match.value_of("IMAGE_URI").unwrap();
        let save_to = fetch_match.value_of("SAVE_TO").unwrap_or("image.aci");
        match commands::fetch::exec(uri, save_to) {
            Ok(res) => println!("{:?}", res),
            Err(err) => print!("{:?}", err),
        }
    }
}
