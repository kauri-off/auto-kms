use std::ptr::null_mut;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::TokenElevation;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::TOKEN_ELEVATION;

use libc;
use std::mem;
use winapi::ctypes::c_void;
use winapi::um::winnt::TOKEN_QUERY;

use std::{io::Error, process::Command};
use std::{thread, time::Duration};

const DELAY: u64 = 5;

const VERSIONS: [(&str, &str); 32] = [
    ("Windows [10/11] Pro", "W269N-WFGWX-YVC9B-4J6C9-T83GX"),
    ("Windows [10/11] Pro N",	"MH37W-N47XK-V7XM9-C7227-GCQG9"),
    ("Windows [10/11] Pro for Workstations",	"NRG8B-VKK3Q-CXVCJ-9G2XF-6Q84J"),
    ("Windows [10/11] Pro for Workstations N",	"9FNHH-K3HBT-3W4TD-6383H-6XYWF"),
    ("Windows [10/11] Pro Education",	"6TP4R-GNPTD-KYYHQ-7B7DP-J447Y"),
    ("Windows [10/11] Pro Education N",	"YVWGF-BXNMC-HTQYQ-CPQ99-66QFC"),
    ("Windows [10/11] Education",	"NW6C2-QMPVW-D7KKK-3GKT6-VCFB2"),
    ("Windows [10/11] Education N",	"2WH4N-8QGBV-H22JP-CT43Q-MDWWJ"),
    ("Windows [10/11] Enterprise",	"NPPR9-FWDCX-D2C8J-H872K-2YT43"),
    ("Windows [10/11] Enterprise N",	"DPH2V-TTNVB-4X9Q3-TJR4H-KHJW4"),
    ("Windows [10/11] Enterprise G",	"YYVX9-NTFWV-6MDM3-9PT4T-4M68B"),
    ("Windows [10/11] Enterprise G N",	"44RPN-FTY23-9VTTB-MP9BX-T84FV"),
    ("Windows 10 Enterprise LTSC [2021/2019]",	"M7XTQ-FN8P6-TTKYV-9D4CC-J462D"),
    ("Windows 10 Enterprise N LTSC [2021/2019]",	"92NFX-8DJQP-P6BBQ-THF9C-7CG2H"),
    ("Windows 8.1 Pro",	"GCRJD-8NW9H-F2CDX-CCM8D-9D6T9"),
    ("Windows 8.1 Pro N",	"HMCNV-VVBFX-7HMBH-CTY9B-B4FXY"),
    ("Windows 8.1 Enterprise",	"MHF9N-XY6XB-WVXMC-BTDCT-MKKG7"),
    ("Windows 8.1 Enterprise N",	"TT4HM-HN7YT-62K67-RGRQJ-JFFXW"),
    ("Windows 8 Pro",	"NG4HW-VH26C-733KW-K6F98-J8CK4"),
    ("Windows 8 Pro N",	"XCVCF-2NXM9-723PB-MHCB7-2RYQQ"),
    ("Windows 8 Enterprise",	"32JNW-9KQ84-P47T8-D8GGY-CWCK7"),
    ("Windows 8 Enterprise N",	"JMNMF-RHW7P-DMY6X-RF3DR-X2BQT"),
    ("Windows 7 Professional",	"FJ82H-XT6CR-J8D7P-XQJJ2-GPDD4"),
    ("Windows 7 Professional N",	"MRPKT-YTG23-K7D7T-X2JMM-QY7MG"),
    ("Windows 7 Professional E",	"W82YF-2Q76Y-63HXB-FGJG9-GF7QX"),
    ("Windows 7 Enterprise",	"33PXH-7Y6KF-2VJC9-XBBR8-HVTHH"),
    ("Windows 7 Enterprise N",	"YDRBP-3D83W-TY26F-D46B2-XCKRJ"),
    ("Windows 7 Enterprise E",	"C29WB-22CC8-VJ326-GHFJW-H9DH4"),
    ("Windows Vista Business",	"YFKBB-PQJJV-G996G-VWGXY-2V3X8"),
    ("Windows Vista Business N",	"HMBQG-8H2RH-C77VX-27R82-VMQBT"),
    ("Windows Vista Enterprise",	"VKK3X-68KWM-X2YGT-QR4M6-4BWMV"),
    ("Windows Vista Enterprise N",	"VTC42-BM838-43QHV-84HX6-XJXKV"),
];

pub struct WinVer {
    version: String,
    gvlk_key: String
}

impl WinVer {
    pub fn get() -> WinVer {
        let user_choice = WinVer::get_user_choice();
        match WinVer::get_by_index(user_choice - 1) {
            Some(t) => {
                return t;
            },
            None => WinVer::get(),
        }
    }

    pub fn get_by_index(i: usize) -> Option<WinVer> {
        let ver = VERSIONS.get(i)?;
        println!("Selected {}", ver.0);
        Some(WinVer {
            version: ver.0.to_string(),
            gvlk_key: ver.1.to_string()
        })
    }

    fn print_me() {
        pretty_print("Select your windows version\n\n", DELAY);
        for (index, version) in VERSIONS.iter().enumerate() {
            println!("{}. {}", index + 1, version.0);
        }
    }

    fn get_user_choice() -> usize {
        WinVer::print_me();
        let choice = pretty_input("Please enter a number: ", DELAY).trim().parse().expect("Error");
        choice
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

pub fn activate(win_ver: &WinVer, server: &SERVER) -> Result<(), Error> {
    pretty_print(&format!("\nCracking: {}\n\n", win_ver.version), DELAY);
    pretty_print("[/] Installing key:", DELAY);
    let ipk = Command::new("C:\\Windows\\System32\\cscript")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .args(&["/ipk", &win_ver.gvlk_key])
        .output()?;


    if ipk.status.success() {
        pretty_print(" success\n\n", DELAY);
    } else {
        pretty_input(&format!(" Error: {:?}", ipk.status.code().unwrap()), DELAY);
        return Ok(());
    }

    pretty_print("[/] Installing KMS Server:", DELAY);
    let skms = Command::new("C:\\Windows\\System32\\cscript")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .args(&["/skms", &server.address()])
        .output()?;

    if skms.status.success() {
        pretty_print(" success\n\n", DELAY);
    } else {
        pretty_input(&format!(" Error: {:?}", skms.status.code().unwrap()), DELAY);
        return Ok(());
    }

    pretty_print("[/] Activating windows:", DELAY);

    let ato = Command::new("C:\\Windows\\System32\\cscript")
        .arg("C:\\Windows\\System32\\slmgr.vbs")
        .args(&["/ato"])
        .output()?;

    if ato.status.success() {
        pretty_print(" Complete", 50);
    } else {
        pretty_input(&format!(" Error: {:?}", ato.status.code().unwrap()), DELAY);
    }

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

pub fn has_admin_privileges() -> bool {
    let mut handle: HANDLE = null_mut();
    unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) };

    let elevation = unsafe { libc::malloc(mem::size_of::<TOKEN_ELEVATION>()) as *mut c_void };
    let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
    let mut ret_size = size;
    unsafe {
        GetTokenInformation(
            handle,
            TokenElevation,
            elevation,
            size as u32,
            &mut ret_size,
        )
    };
    let elevation_struct: TOKEN_ELEVATION = unsafe{ *(elevation as *mut TOKEN_ELEVATION)};

    if !handle.is_null() {
        unsafe {
            CloseHandle(handle);
        }
    }

    elevation_struct.TokenIsElevated == 1
}