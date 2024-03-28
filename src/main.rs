use auto_kms::{activate, pretty_input, pretty_print, SERVER, WIN_VER};


fn print_menu() {
    let term = console::Term::stdout();
    term.clear_screen().expect("Не удалось очистить консоль");
    pretty_print("Auto KMS\n\n", 5);
    pretty_print("1. Activate windows\n", 5);
    pretty_print("2. Fast activate windows [10/11 Pro]\n", 5);
    pretty_print("3. Exit\n\n", 5);
}

fn main() {
    while true {
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
    let ver = WIN_VER::WINDOWS_10_11_PRO;
    let server = SERVER{ip: "kdavp.mooo.com".to_string()};

    match activate(&ver, &server) {
        Ok(_) => (),
        Err(E) => {
            println!("{}", E)
        },
    };
    pretty_input("Complete", 50);
}

fn activate_windows() {
    let ver = WIN_VER::get();
    let server = SERVER::get();

    match activate(&ver, &server) {
        Ok(_) => (),
        Err(E) => {
            println!("{}", E)
        },
    };
    pretty_input("Complete", 50);
}