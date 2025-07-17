use std::{io::Write, process::Command};

use dialoguer::Confirm;

pub fn activate(key: &str, server: &str) {
    print!("[/] Installing key: ");
    std::io::stdout().flush().unwrap();

    let ipk = Command::new("C:\\Windows\\System32\\cscript")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .args(&["/ipk", key])
        .output()
        .unwrap();

    if ipk.status.success() {
        println!("success\n\n");
    } else {
        println!(" Error: {:?}", ipk.status.code().unwrap());
        Confirm::new().interact().unwrap();
        return;
    }

    print!("[/] Installing KMS Server: ");
    std::io::stdout().flush().unwrap();
    let skms = Command::new("C:\\Windows\\System32\\cscript")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .args(&["/skms", server])
        .output()
        .unwrap();

    if skms.status.success() {
        println!("success\n\n");
    } else {
        println!("Error: {:?}", skms.status.code().unwrap());
        Confirm::new().interact().unwrap();
        return;
    }

    print!("[/] Activating windows: ");
    std::io::stdout().flush().unwrap();

    let ato = Command::new("C:\\Windows\\System32\\cscript")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .args(&["/ato"])
        .output()
        .unwrap();

    if ato.status.success() {
        println!("Complete");
    } else {
        println!("Error: {:?}", ato.status.code().unwrap());
    }
}
