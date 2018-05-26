extern crate config;
extern crate reddit_refresh_rust;
extern crate indexmap;

use config::{Config};
use reddit_refresh_rust::reserializer::{reserialize};
use reddit_refresh_rust::subparser::{get_results, SubResult};
use reddit_refresh_rust::pushbullet::{get_devices, send_push_link};
use std::fs::File;
use std::path::Path;
use std::io::prelude::Write;
use std::io::stdin;
use std::io::stdout;
use std::{thread, time};

const CONF_TOKEN: &str = "user_info.token";
const CONF_INTERVAL: &str = "program_config.interval";
const SUBS: &str = "subreddits";
const LAST_RESULT: &str = "last_result";
const DEVICES: &str = "devices";

fn main() {
    let mut settings = Config::new();
    
    if Path::exists(Path::new("Settings.toml")){
        settings.merge(config::File::with_name("Settings")).unwrap();
    }

    get_user_settings(&mut settings);

    let interval = settings.get_str(CONF_INTERVAL).unwrap();

    let interval: f64 = interval.parse().expect("Interval not a number");

    let interval = (interval * 60.0) as u64;

    let wait_time = time::Duration::from_secs(interval);

    loop{

        println!("Checking results (Ctrl-C to stop)");

        for (subreddit, searches) in settings.get_table(SUBS).unwrap(){
            for search in searches.into_array().unwrap(){
                let search = search.into_str().unwrap();
                let result = get_results(subreddit.clone(), 
                    search.clone()).unwrap();
                let last_path = format!("{}.{}_{}", LAST_RESULT, subreddit, search);
                handle_result(&mut settings, result, last_path);
            }
        }

        let output = reserialize(&settings);
        let mut file = File::create("Settings.toml").unwrap();
        file.write_all(output.as_bytes()).unwrap();

        thread::sleep(wait_time);

    }

}

fn handle_result(config: &mut Config, (link, title): SubResult, last_path:String){
    let mut devices = Vec::new();
    for (_, id) in config.get_table(DEVICES).unwrap(){
        devices.push(id.into_str().unwrap());
    }
    let token = config.get_str(CONF_TOKEN).unwrap();
    match config.get::<String>(&last_path){
        Ok(last_res) => {
            if last_res != link{
                config.set(&last_path, link.clone()).unwrap();
                send_push_link(devices, &token, (link, title))
            }
        }
        Err(_) => {
            config.set(&last_path, link.clone()).unwrap();
            send_push_link(devices, &token, (link, title))
        }
    };
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

    match config.get_table(DEVICES){
        Ok(map) => {
            if map.is_empty(){
                conf_devices(config);
            }
        },
        Err(_) => conf_devices(config)
    }
}

fn get_user_setting(config: &mut Config, msg: &str, setting: &str){
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut item = String::new();
    stdin().read_line(&mut item).unwrap();
    config.set(setting, item.trim()).unwrap();
}

fn conf_devices(config: &mut Config){
    let token = config.get::<String>(CONF_TOKEN).unwrap();
    let devices = get_devices(token);
    let mut devices_vec = Vec::new();
    for device in devices{
        devices_vec.push(device);
    }
    println!("Devices available to be pushed to: ");
    for (ind, (nick, _)) in devices_vec.clone().into_iter().enumerate(){
        println!("{}: {}", ind, nick);
    }
    print!("Enter devices to be pushed to (e.g. 1, 2): ");
    stdout().flush().unwrap();
    let mut device_inds = String::new();
    stdin().read_line(&mut device_inds).unwrap();
    for index in device_inds.trim().replace(" ", "").split(","){
        let index = index.parse::<usize>().unwrap();
        let device = &devices_vec.get(index).unwrap();
        let mut toml_friendly_nick = &device.0;
        let key_2 = toml_friendly_nick.replace(" ", "_");
        let key = format!("{}.{}", DEVICES, key_2);
        config.set(&key[..], &device.1[..]).unwrap();
    }
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
