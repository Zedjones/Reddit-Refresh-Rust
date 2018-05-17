extern crate config;
extern crate reddit_refresh_rust;

use std::collections::HashMap;
use config::{Config, File};
use reddit_refresh_rust::reserializer::{reserialize};

fn main() {
    let mut settings = Config::new();
    
    settings.merge(File::with_name("Settings")).unwrap();

    /*

    let subreddits : HashMap<String, config::Value> = 
        settings.get_table("subreddits").unwrap();

    for (key, value) in subreddits{
        for inner_value in value.into_array(){
            for item in inner_value.iter(){
                let item = item.clone().into_str();
                println!("{}: {}", key, item.unwrap());
            }
        }
    }
    */

    settings.set("user_info.keys", "10543sd").unwrap();

    let test = vec!["dog", "man"];

    settings.set("subreddits.keyboard", test).unwrap();
    /*
    let user_info: HashMap<String, config::Value> = 
        settings.get_table("user_info").unwrap();

    for (key, value) in user_info{
        println!("{}: {}", key, value.into_str().unwrap());
    }

    let program_config: HashMap<String, config::Value> = 
        settings.get_table("program_config").unwrap();

    for (key, value) in program_config{
        println!("{}: {}", key, value.into_str().unwrap());
    }

    let devices: HashMap<String, config::Value> = 
        settings.get_table("devices").unwrap();

    for (key, value) in devices{
        println!("{}: {}", key, value.into_str().unwrap());
    }
    */

    reserialize(settings);
}