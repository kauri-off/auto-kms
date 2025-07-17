use auto_kms::activate;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
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
        eprintln!("Restart the program with administrator rights");
        Confirm::new().interact().unwrap();
        return;
    }

    let config_str = include_str!("../keys.json");
    let versions: Versions = serde_json::from_str(config_str).unwrap();

    let options: Vec<String> = versions.versions.iter().map(|t| t.name.clone()).collect();
    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Version")
        .items(&options)
        .interact()
        .unwrap();

    let server: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("KMS Server")
        .interact_text()
        .unwrap();

    activate(&versions.versions[selected].key, &server);

    Confirm::new().interact().unwrap();
}
