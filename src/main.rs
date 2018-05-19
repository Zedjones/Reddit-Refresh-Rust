extern crate config;
extern crate reddit_refresh_rust;

use config::{Config};
use reddit_refresh_rust::reserializer::{reserialize};
use reddit_refresh_rust::subparser::get_results;
use std::fs::File;
use std::io::prelude::Write;
use std::io::stdin;
use std::io::stdout;

const CONF_TOKEN: &str = "user_info.token";
const CONF_INTERVAL: &str = "program_config.interval";

fn main() {
    let mut settings = Config::new();
    
    settings.merge(config::File::with_name("Settings")).unwrap();

    settings.set("user_info.keys", "10543sd").unwrap();

    let test = vec!["dog", "man"];

    settings.set("subreddits.keyboard", test).unwrap();

    settings.set("users.zedjones", "me").unwrap();

    get_user_settings(&mut settings);

    let output = reserialize(settings);

    let mut file = File::create("Test.toml").unwrap();

    file.write_all(output.as_bytes()).unwrap();

    get_results("mechanicalkeyboards".to_string(), "Planck".to_string()).unwrap();

    //println!("{}", output);
}

fn get_user_settings(config: &mut Config){
    match config.get::<String>(CONF_TOKEN){
        Ok(_) => (),
        Err(_) => {
            print!("Please enter your Pushbullet API token: ");
            stdout().flush().unwrap();
            let mut token = String::new();
            stdin().read_line(&mut token).unwrap();
            config.set(CONF_TOKEN, token.trim()).unwrap();
        }
    };

    match config.get::<String>(CONF_INTERVAL){
        Ok(_) => (),
        Err(_) => {
            print!("Interval to check for new results (in minutes): ");
            stdout().flush().unwrap();
            let mut interval = String::new();
            stdin().read_line(&mut interval).unwrap();
            config.set(CONF_INTERVAL, interval.trim()).unwrap();
        }
    };

}