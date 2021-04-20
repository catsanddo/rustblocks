use std::process::Command;
use std::time::Instant;
use toml::Value;

pub struct Block {
    pub command: String,
    pub interval: u64,
    pub instant: Instant,
    pub value: String,
}

impl Block {
    pub fn execute(&self) -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .output().expect("Could not execute command");
        let mut output = String::from_utf8_lossy(&output.stdout).to_string();
        output.pop();
        output
    }
}

pub fn load_config(config: &Value) -> (String, Vec::<Block>) {
    let delim = if let Value::String(deliminator) = &config["deliminator"] {
        deliminator.to_string()
    } else {
        String::from("|")
    };

    let mut blocks: Vec::<Block> = Vec::new();
    if let Value::Array(b) = &config["block"] {
        for block in b.as_slice() {
            let command = String::from(block["command"].as_str().unwrap());
            let value = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output().expect("Could not execute command");
            let mut value = String::from_utf8_lossy(&value.stdout).to_string();
            value.pop();
            blocks.push(Block { 
                command: command,
                interval: block["interval"].as_integer().unwrap() as u64,
                instant: Instant::now(),
                value: value,
            });
        }
    }
    
    (delim, blocks)
}
