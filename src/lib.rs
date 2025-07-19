use std::io::{self, Write};
use std::process::{Command, Output};

use dialoguer::Confirm;

fn run_slmgr(args: &[&str], description: &str) -> io::Result<bool> {
    print!("[/] {}... ", description);
    io::stdout().flush()?;

    let output: Output = Command::new("C:\\Windows\\System32\\cscript")
        .arg("//nologo")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .args(args)
        .output()?;

    if output.status.success() {
        println!("success\n");
        Ok(true)
    } else {
        eprintln!(
            "\n[!] Error: {}\n{}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        );

        Confirm::new()
            .with_prompt("Continue anyway?")
            .default(false)
            .interact()
            .unwrap_or(false);

        Ok(false)
    }
}

pub fn activate(key: &str, server: &str) -> io::Result<()> {
    run_slmgr(&["/upk"], "Uninstalling existing product key")?;

    if !run_slmgr(&["/ipk", key], &format!("Installing product key: {}", key))? {
        return Ok(());
    }

    if !run_slmgr(
        &["/skms", server],
        &format!("Setting KMS server: {}", server),
    )? {
        return Ok(());
    }

    if !run_slmgr(&["/ato"], "Activating Windows")? {
        return Ok(());
    }

    Command::new("C:\\Windows\\System32\\cscript")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .arg("/dlv")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Ok(())
}
