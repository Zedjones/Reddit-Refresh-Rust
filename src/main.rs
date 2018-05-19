extern crate config;
extern crate reddit_refresh_rust;

use config::{Config};
use reddit_refresh_rust::reserializer::{reserialize};
use reddit_refresh_rust::subparser::get_results;
use std::fs::File;
use std::io::prelude::Write;

fn main() {
    let mut settings = Config::new();
    
    settings.merge(config::File::with_name("Settings")).unwrap();

    settings.set("user_info.keys", "10543sd").unwrap();

    let test = vec!["dog", "man"];

    settings.set("subreddits.keyboard", test).unwrap();

    settings.set("users.zedjones", "me").unwrap();

    let output = reserialize(settings);

    let mut file = File::create("Test.toml").unwrap();

    file.write_all(output.as_bytes()).unwrap();

    get_results("mechanicalkeyboards".to_string(), "Planck".to_string()).unwrap();

    //println!("{}", output);
}