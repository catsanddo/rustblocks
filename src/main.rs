extern crate toml;

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::process::Command;
use std::env;
use toml::Value;
use rustblocks::block;

fn main() {
    let mut home = env::var("HOME").unwrap();
    home.push_str("/.rustblocks.toml");
    let config = std::fs::read_to_string(&home).expect("Could not read config file!");
    let config = config.as_str().parse::<Value>().unwrap();

    let (delim, mut blocks) = block::load_config(&config);
    let delim = " ".to_owned() + &delim + " ";

    loop {
        let mut output = String::new();

        let last_index = blocks.len() - 1;
        for (i, block) in blocks.iter_mut().enumerate() {
            if block.instant.elapsed().as_secs() > block.interval {
                block.instant = Instant::now();
                block.value = block.execute();
            }
            output.push_str(&block.value);
            if i != last_index {
                output.push_str(&delim);
            }
        }
        Command::new("xsetroot")
            .arg("-name")
            .arg(&output)
            .output()
            .expect("Could not call xsetroot!");

        sleep(Duration::new(30, 0));
    }
}
