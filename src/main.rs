#![recursion_limit = "1024"]

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate reqwest;
extern crate tar;

mod commands;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}

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
        .subcommand(SubCommand::with_name("unpack")
            .about("Unpacks local image")
            .arg(Arg::with_name("LOCAL_IMAGE")
                .required(true)
                .index(1))
            .arg(Arg::with_name("UNPACK_TO")
                .required(false)
                .index(2)))
        .get_matches();

    if let Some(fetch_match) = matches.subcommand_matches("fetch") {
        let uri = fetch_match.value_of("IMAGE_URI").unwrap();
        let save_to = fetch_match.value_of("SAVE_TO").unwrap_or("image.aci");
        match commands::fetch::exec(uri, save_to) {
            Ok(res) => println!("{}", res),
            Err(err) => {
                println!("{}", err);
                ::std::process::exit(1);
            }
        }
    }

    if let Some(unpack_match) = matches.subcommand_matches("unpack") {
        let image = unpack_match.value_of("LOCAL_IMAGE").unwrap();
        let unpack_to = unpack_match.value_of("UNPACK_TO").unwrap_or("unpacked.img");
        match commands::unpack::exec(image, unpack_to) {
            Ok(res) => println!("{}", res),
            Err(err) => {
                println!("{}", err);
                ::std::process::exit(1);
            }
        }
    }
}
