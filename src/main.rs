extern crate clap;
extern crate yaml_rust;
use clap::{Arg, App};
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

struct Job {
    family: String,
    datasets: Vec<String>,
    schedule: String,
    keep: i64,
    recursive: bool,
}

fn get_jobs(yaml: Vec<Yaml>) -> Vec<Job> {
    let mut jobs: Vec<Job> = Vec::new();
    if yaml.len() == 0 {
        panic!("The config file is empty!");
    }
    let yaml_jobs = (&yaml[0]).as_vec().unwrap();
    for yaml_job in yaml_jobs {
        let mut datasets: Vec<String> = Vec::new();
        let yaml_datasets = yaml_job["datasets"].as_vec().unwrap();
        for dataset in yaml_datasets {
            datasets.push(String::from(dataset.as_str().unwrap()));
        }
        let job = Job {
            family: String::from(yaml_job["family"].as_str().unwrap()),
            keep: yaml_job["keep"].as_i64().unwrap(),
            recursive: yaml_job["recursive"].as_bool().unwrap(),
            schedule: String::from(yaml_job["schedule"].as_str().unwrap()),
            datasets: datasets,
        };
        jobs.push(job);
    }
    jobs
}

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
    let jobs = get_jobs(config);
    for job in jobs {
        println!("Job family: {}", job.family);
        for dataset in job.datasets {
            println!("Dataset: {}", dataset);
        }
    }
}
