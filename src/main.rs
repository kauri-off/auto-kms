use std::{
    env,
    process::{exit, Command},
};

use auto_kms::activate;
use dialoguer::{Confirm, Input, Select};
use serde::{Deserialize, Serialize};

use crate::is_elevated::is_elevated;
mod is_elevated;

#[derive(Serialize, Deserialize)]
struct Versions {
    versions: Vec<Version>,
}

#[derive(Serialize, Deserialize)]
struct Version {
    name: String,
    key: String,
}

fn main() {
    if !is_elevated() {
        let exe = env::current_exe().unwrap();
        let args: Vec<String> = env::args().skip(1).collect();

        let mut ps_command = format!("Start-Process '{}' -Verb runAs", exe.display());

        if !args.is_empty() {
            ps_command.push_str(&format!(" -ArgumentList '{}'", args.join(" ")));
        }

        let status = Command::new("powershell")
            .args(&["-Command", &ps_command])
            .status()
            .expect("Failed to relaunch as admin");

        if status.success() {
            exit(0);
        } else {
            eprintln!("Failed to relaunch with administrator rights.");
            exit(1);
        }
    }

    let config_str = include_str!("../keys.json");
    let versions: Versions = serde_json::from_str(config_str).unwrap();

    let options: Vec<String> = versions.versions.iter().map(|t| t.name.clone()).collect();
    let selected = Select::new()
        .with_prompt("Version")
        .items(&options)
        .interact()
        .unwrap();

    let server: String = Input::new()
        .with_prompt("KMS Server")
        .interact_text()
        .unwrap();

    if let Err(e) = activate(&versions.versions[selected].key, &server) {
        eprintln!("{}", e);
    }

    Confirm::new().interact().unwrap();
}
