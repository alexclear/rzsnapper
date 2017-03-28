extern crate clap;
extern crate yaml_rust;
use clap::{Arg, App};
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

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

    let config_name = matches.value_of("config").unwrap_or("/opt/local/zsnapper/etc/zsnapper.yml");
    println!("Value for config: {}", config_name);

    let file = File::open(config_name).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let config = YamlLoader::load_from_str(contents.as_str()).unwrap();
}
