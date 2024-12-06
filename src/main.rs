use auto_kms::{activate, has_admin_privileges, pretty_input, pretty_print, SERVER, WinVer};


fn print_menu() {
    let term = console::Term::stdout();
    term.clear_screen().expect("Не удалось очистить консоль");
    pretty_print("Auto KMS\n\n", 5);
    pretty_print("1. Activate windows\n", 5);
    pretty_print("2. Fast activate windows [10/11 Pro]\n", 5);
    pretty_print("3. Exit\n\n", 5);
}

fn main() {
    if !has_admin_privileges() {
        pretty_input("Restart the program with administrator rights", 20);
        return;
    }
    loop {
        print_menu();
        match pretty_input("Select: ", 5).trim() {
            "1" => {activate_windows();}
            "2" => {fast_activate();}
            "3" => {return;}
            _ => {

            }
        }
    }
}

fn fast_activate() {
    let ver = WinVer::get_by_index(0).unwrap();
    let server = SERVER{ip: "kms.loli.best".to_string()};

    match activate(&ver, &server) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e)
        },
    };
}

fn activate_windows() {
    let ver = WinVer::get();
    let server = SERVER::get();

    match activate(&ver, &server) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e)
        },
    };
}