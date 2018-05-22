extern crate config;
extern crate reddit_refresh_rust;

use config::{Config};
use reddit_refresh_rust::reserializer::{reserialize};
use reddit_refresh_rust::subparser::get_results;
use reddit_refresh_rust::pushbullet::{get_devices, send_push_link};
use std::fs::File;
use std::path::Path;
use std::io::prelude::Write;
use std::io::stdin;
use std::io::stdout;
use std::collections::HashMap;

const CONF_TOKEN: &str = "user_info.token";
const CONF_INTERVAL: &str = "program_config.interval";
const SUBS: &str = "subreddits";

fn main() {
    let mut settings = Config::new();
    
    if Path::exists(Path::new("Settings.toml")){
        settings.merge(config::File::with_name("Settings")).unwrap();
    }

    get_user_settings(&mut settings);

    for (subreddit, searches) in settings.get_table(SUBS).unwrap(){
        for search in searches.into_array().unwrap(){
            println!("{}:{}", subreddit, search);
        }
    }

    let output = reserialize(settings);

    let mut file = File::create("Test.toml").unwrap();

    file.write_all(output.as_bytes()).unwrap();

}

fn get_user_settings(config: &mut Config){
    match config.get::<String>(CONF_TOKEN){
        Ok(_) => (),
        Err(_) => {
            get_user_setting(config, "Please enter your Pushbullet \
            API token: ", CONF_TOKEN);
        }
    };

    match config.get::<String>(CONF_INTERVAL){
        Ok(_) => (),
        Err(_) => {
            get_user_setting(config, "Interval to check for new results \
            (in minutes): ", CONF_INTERVAL);
        }
    };

    match config.get_table(SUBS){
        Ok(map) => {
            if map.is_empty() {
                get_subreddits(config);
            }
        }, 
        Err(_) => get_subreddits(config)
    };
}

fn get_user_setting(config: &mut Config, msg: &str, setting: &str){
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut item = String::new();
    stdin().read_line(&mut item).unwrap();
    config.set(setting, item.trim()).unwrap();
}

fn get_subreddits(config: &mut Config){
    println!("Hit enter to stop inputting subreddits");
    loop{
        print!("Enter a subreddit to search: ");
        stdout().flush().unwrap();
        let mut subreddit = String::new();
        stdin().read_line(&mut subreddit).unwrap();
        subreddit = subreddit.trim().to_string();
        if subreddit == "" { break; }
        println!("Hit enter to stop inputting searches for {}", subreddit);
        let mut searches: Vec<String> = Vec::new();
        loop{
            print!("Enter a search term: ");
            stdout().flush().unwrap();
            let mut search = String::new();
            stdin().read_line(&mut search).unwrap();
            search = search.trim().to_string();
            if search == "" { break; }
            searches.push(search);
        }
        let key = format!("subreddits.{}", subreddit);
        match config.set(&key, searches){
            Ok(_) => (),
            Err(_) => println!("Invalid subreddit entered")
        };
    }
}