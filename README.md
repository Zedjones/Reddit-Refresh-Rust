# Reddit-Refresh-Rust ![License: GPL v2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)

This is a rework and cleanup of my Python program, [Reddit-Refresh](https://git.io/vNTps).

`Reddit-Refresh-Rust` is a program that scans a provided subreddit (or set of subreddits) for one or more search terms, checks for new results on a provided time interval, and notifies the user using the [Pushbullet API](https://docs.pushbullet.com). Upon first run, it will prompt the user for their API token (located on [this site](https://www.pushbullet.com/#settings/account) > Access Tokens).    

Example use cases:
Getting news updates on a certain topic from /r/news, checking for a keycap set on /r/mechmarket, or checking for a certain game on /r/gamedeals. 

## Table of Contents

<!-- vim-markdown-toc GFM -->
* [Installation](#installation)
* [Configuration](#configuration)
* [Future Features](#future-features)

## Installation

1. Clone the repo into whatever folder you want to use.
2. Make sure you have [Rust and Cargo installed](https://www.rust-lang.org/en-US/install.html).
3. Run `cargo install` while in the root directory of the repository. 
4. You're good to go! Just run `reddit_refresh_online` in your shell. 

## Configuration

Upon first run, you will be prompted with options to configure Pushbullet pushes and searches. However, you can manually edit the file, located in the directory you ran the executable in, called `Settings.toml`.

```sh
[user_info]
token = API_TOKEN #your api token goes here

[devices]
#this is your device's name and unique id for Pushbullet
#can have as many entries as needed
DEVICE_NAME = DEVICE_ID 

[subreddits]
#subreddit followed by search terms separated by commas
#can have as many entries as needed
SUBREDDIT = [TERM_1,TERM_2]

[program_config]
#how often to check for new search results, you can use decimals (e.g .1)
interval = TIME_IN_MINUTES
```

## Future Features

* Filter based on flair
* Send to all devices as one push
* Add option to append flair to title
* Add option to append date and time of post to title
