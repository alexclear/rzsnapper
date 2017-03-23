extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rzsnapper")
        .author("Alex Chistyakov <alexclear@gmail.com>")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file")
                 .takes_value(true))
        .arg(Arg::with_name("v").short("v").help("Enables verbose output").takes_value(false))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("/opt/local/zsnapper/etc/zsnapper.yml");
    println!("Value for config: {}", config);
}
