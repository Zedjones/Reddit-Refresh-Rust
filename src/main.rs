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

    //create new config and open settings
    let mut settings = Config::new();
    
    if Path::exists(Path::new("Settings.toml")){
        settings.merge(config::File::with_name("Settings")).unwrap();
    }

    get_user_settings(&mut settings);

    //get interval as seconds 
    let interval = settings.get_str(CONF_INTERVAL).unwrap();
    let interval: f64 = interval.parse().expect("Interval not a number");
    let interval = (interval * 60.0) as u64;
    let wait_time = time::Duration::from_secs(interval);

    loop{

        println!("Checking results (Ctrl-C to stop)");

        //get the results for every subreddit and each search within that 
        for (subreddit, searches) in settings.get_table(SUBS).unwrap(){
            for search in searches.into_array().unwrap(){
                let search = search.into_str().unwrap();
                let result = get_results(subreddit.clone(), 
                    search.clone()).unwrap();
                //keep track of last result 
                let last_path = format!("{}.{}_{}", LAST_RESULT, subreddit, search.replace(" ", "_"));
                handle_result(&mut settings, result, last_path);
            }
        }

        //reserialize the configuration and write it to settings.toml
        let output = reserialize(&settings);
        let mut file = File::create("Settings.toml").unwrap();
        file.write_all(output.as_bytes()).unwrap();

        //wait the interval to check for results again 
        thread::sleep(wait_time);

    }

}

/**
 * Handle a single result (a link and title), both output
 * and configuration changes
 * @param config - the settings configuration to write to/read from
 * @param (link, title) - the SubResult tuple from a subreddit search
 * @param last_path - the config key to write the latest value to 
 */
fn handle_result(config: &mut Config, (link, title): SubResult, last_path:String){
    let mut devices = Vec::new();
    //we don't care about the device name, so we'll wildcard it     
    for (_, id) in config.get_table(DEVICES).unwrap(){
        devices.push(id.into_str().unwrap());
    }
    let token = config.get_str(CONF_TOKEN).unwrap();
    match config.get::<String>(&last_path){
        //if there's a value for the last value, only send a push
        //for a new result if it's different
        Ok(last_res) => {
            if last_res != link{
                config.set(&last_path, link.clone()).unwrap();
                send_push_link(devices, &token, (link, title))
            }
        }
        //if there's no old value, send a push for new result
        Err(_) => {
            config.set(&last_path, link.clone()).unwrap();
            send_push_link(devices, &token, (link, title))
        }
    };
}

/** 
 * Gets all necessary user input if no present in the settings 
 * file already
 * @param config - the configuration to read/write from
 */
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

    //if there are no subreddits, we'll get those
    match config.get_table(SUBS){
        Ok(map) => {
            if map.is_empty() {
                get_subreddits(config);
            }
        }, 
        Err(_) => get_subreddits(config)
    };

    //if there are no devies, we'll get those too
    match config.get_table(DEVICES){
        Ok(map) => {
            if map.is_empty(){
                conf_devices(config);
            }
        },
        Err(_) => conf_devices(config)
    }
}

/**
 * Get a single user setting
 * @param config - the configuration to write to
 * @param msg - the message to print to the user
 * @param setting - the configuration key to write to
 */
fn get_user_setting(config: &mut Config, msg: &str, setting: &str){
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut item = String::new();
    stdin().read_line(&mut item).unwrap();
    config.set(setting, item.trim()).unwrap();
}

/**
 * Get the configuration for the user's devices
 * @param config - the configuration to read/write 
 */
fn conf_devices(config: &mut Config){
    let token = config.get::<String>(CONF_TOKEN).unwrap();
    let devices = get_devices(token);
    let mut devices_vec = Vec::new();
    for device in devices{
        devices_vec.push(device);
    }
    //print out devices available, we only care about index and nick,
    //not the id for printing them out
    println!("Devices available to be pushed to: ");
    for (ind, (nick, _)) in devices_vec.clone().into_iter().enumerate(){
        println!("{}: {}", ind, nick);
    }
    //get the devies the user wants to push to by index
    print!("Enter devices to be pushed to (e.g. 1, 2): ");
    stdout().flush().unwrap();
    let mut device_inds = String::new();
    stdin().read_line(&mut device_inds).unwrap();
    for index in device_inds.trim().replace(" ", "").split(","){
        let index = index.parse::<usize>().unwrap();
        let device = &devices_vec.get(index).unwrap();
        let mut toml_friendly_nick = &device.0;
        //make sure each nick has no spaces
        let key_2 = toml_friendly_nick.replace(" ", "_");
        let key = format!("{}.{}", DEVICES, key_2);
        //write key(devices path + nick) and value(id)
        config.set(&key[..], &device.1[..]).unwrap();
    }
}

/**
 * Get the subreddits and search queries from the user 
 * @param config - the configuration file to write to
 */
fn get_subreddits(config: &mut Config){
    println!("Hit enter to stop inputting subreddits");
    //get as many subreddits as the user wants 
    loop{
        print!("Enter a subreddit to search: ");
        stdout().flush().unwrap();
        let mut subreddit = String::new();
        stdin().read_line(&mut subreddit).unwrap();
        subreddit = subreddit.trim().to_string();
        //if enter was pressed, let's exit 
        if subreddit == "" { break; }
        //get as many search queries as the user wants 
        println!("Hit enter to stop inputting searches for {}", subreddit);
        let mut searches: Vec<String> = Vec::new();
        loop{
            print!("Enter a search term: ");
            stdout().flush().unwrap();
            let mut search = String::new();
            stdin().read_line(&mut search).unwrap();
            search = search.trim().to_string();
            //if enter is pressed, let's exit
            if search == "" { break; }
            searches.push(search);
        }
        //write out the key and searches to the configuration
        let key = format!("subreddits.{}", subreddit);
        match config.set(&key, searches){
            Ok(_) => (),
            Err(_) => println!("Invalid subreddit entered")
        };
    }
}
