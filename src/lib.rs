use std::{io::Error, process::Command};
use std::{thread, time::Duration};

const DELAY: u64 = 5;

#[derive(Clone)]
#[derive(Debug)]
pub enum WIN_VER {
    WINDOWS_10_11_PRO
}

impl WIN_VER {
    pub fn GVLK(&self) -> &'static str {
        match self {
            WIN_VER::WINDOWS_10_11_PRO => "W269N-WFGWX-YVC9B-4J6C9-T83GX"
        }
    }

    pub fn get() -> WIN_VER {
        let user_choice = WIN_VER::get_user_choice();

        match WIN_VER::values().get(user_choice - 1) {
            Some(selected_version) => {
                println!("Selected {:?}", selected_version);
                selected_version.clone()
            }
            None => WIN_VER::get(),
        }
    }

    fn print_me() {
        pretty_print("Select your windows version\n\n", DELAY);
        for (index, version) in WIN_VER::values().iter().enumerate() {
            pretty_print(&format!("{}. {:#?}\n", index + 1, version), DELAY);
        }
    }

    fn get_user_choice() -> usize {
        WIN_VER::print_me();
        let choice = pretty_input("Please enter a number: ", DELAY).trim().parse().expect("Error");
        choice
    }

    fn values() -> Vec<WIN_VER> {
        vec![
            WIN_VER::WINDOWS_10_11_PRO
        ]
    }
}

pub struct SERVER {
    pub ip: String,
}

impl SERVER {
    pub fn address(&self) -> String {
        return format!("{}", self.ip.trim());
    }

    pub fn get() -> Self {
        let ip = pretty_input("KMS Server: ", DELAY);

        SERVER {
            ip: ip
        }
    }
}

pub fn activate(win_ver: &WIN_VER, server: &SERVER) -> Result<(), Error> {
    pretty_print("[/] Key set", DELAY);
    // Command::new("C:\\Windows\\System32\\cscript")
    //     .arg("C:\\Windows\\System32\\slmgr.vbs")
    //     .args(&["/ipk", win_ver.GVLK()])
    //     .output()?;
    pretty_print(" success\n\n", DELAY);

    pretty_print("[/] KMS Server set", DELAY);
    // Command::new("C:\\Windows\\System32\\cscript")
    //     .arg("C:\\Windows\\System32\\slmgr.vbs")
    //     .args(&["/skms", &server.address()])
    //     .output()?;
    pretty_print(" success\n\n", DELAY);
    
    pretty_print("[/] Activating windows ", DELAY);

    // Command::new("C:\\Windows\\System32\\cscript")
    //     .arg("C:\\Windows\\System32\\slmgr.vbs")
    //     .args(&["/ato"])
    //     .output()?;

    Ok(())
}

pub fn pretty_print(string: &str, delay: u64) {
    for i in string.chars() {
        eprint!("{}", i);
        thread::sleep(Duration::from_millis(delay));
    }

}

pub fn pretty_input(prefix: &str, delay: u64) -> String {
    pretty_print(prefix, delay);
    let mut line = String::new();


    std::io::stdin().read_line(&mut line).unwrap();

    line
}