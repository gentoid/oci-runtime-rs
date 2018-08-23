mod commands;

fn main() {
    match commands::fetch::exec("some:container") {
        Ok(res) => println!("{:?}", res),
        Err(err) => print!("{:?}", err),
    }
}
