extern crate config;
extern crate reddit_refresh_rust;

use config::{Config, File};
use reddit_refresh_rust::reserializer::{reserialize};

fn main() {
    let mut settings = Config::new();
    
    settings.merge(File::with_name("Settings")).unwrap();

    settings.set("user_info.keys", "10543sd").unwrap();

    let test = vec!["dog", "man"];

    settings.set("subreddits.keyboard", test).unwrap();

    let output = reserialize(settings);
    println!("{}", output);
}