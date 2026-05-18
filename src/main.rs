use std::sync::mpsc;
use std::time::Duration;

use clap::{Parser, Subcommand};
use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible, SetForegroundWindow,
    ShowWindow, SW_RESTORE,
};
use winrt_toast_reborn::{Toast, ToastManager};

#[derive(Parser)]
#[command(name = "knock-knock", about = "Terminal AI agent notification tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a notification
    Notify {
        /// Notification body message
        message: String,

        /// Notification title (defaults to terminal window title)
        #[arg(short, long)]
        title: Option<String>,

        /// Mark as urgent (persistent toast with sound)
        #[arg(short, long)]
        urgent: bool,

        /// Source identifier (e.g., terminal name)
        #[arg(short, long)]
        source: Option<String>,
    },

    /// Bring a terminal window to foreground
    Focus {
        /// Window title to search for
        #[arg(short, long)]
        title: String,
    },
}

const AUMID: &str = "knock-knock";

fn ensure_aumid_registered() {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Classes\AppUserModelId";
    let app_model_id = match hkcu.create_subkey(path) {
        Ok((key, _)) => match key.create_subkey(AUMID) {
            Ok((app_key, _)) => app_key,
            Err(_) => return,
        },
        Err(_) => return,
    };

    let _ = app_model_id.set_value("DisplayName", &"knock-knock");
}

fn get_terminal_title() -> Option<String> {
    let mut buf = [0u16; 512];
    let len = unsafe {
        windows_sys::Win32::System::Console::GetConsoleTitleW(buf.as_mut_ptr(), buf.len() as u32)
    };
    if len == 0 {
        return None;
    }
    String::from_utf16(&buf[..len as usize]).ok()
}

fn activate_window_by_title(target: &str) -> bool {
    struct SearchData {
        target: Vec<u16>,
        found: HWND,
    }

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            let data = &mut *(lparam as *mut SearchData);

            if IsWindowVisible(hwnd) == 0 {
                return 1;
            }

            let len = GetWindowTextLengthW(hwnd);
            if len == 0 {
                return 1;
            }

            let mut buf = vec![0u16; (len + 1) as usize];
            let actual = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
            if actual == 0 {
                return 1;
            }
            buf.truncate(actual as usize);

            if buf.windows(data.target.len()).any(|w| w == data.target) {
                data.found = hwnd;
                return 0;
            }

            1
        }
    }

    let target_wide: Vec<u16> = target.encode_utf16().collect();
    let mut data = SearchData {
        target: target_wide,
        found: std::ptr::null_mut(),
    };

    unsafe {
        EnumWindows(Some(enum_callback), &mut data as *mut SearchData as LPARAM);

        if !data.found.is_null() {
            ShowWindow(data.found, SW_RESTORE);
            SetForegroundWindow(data.found);
            return true;
        }
    }

    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Notify {
            message,
            title,
            urgent,
            source,
        } => {
            let title = title
                .or_else(|| get_terminal_title())
                .unwrap_or_else(|| "knock-knock".to_string());

            let (tx, rx) = mpsc::channel();
            let tx_activated = tx.clone();
            let tx_dismissed = tx.clone();
            let title_for_callback = title.clone();

            ensure_aumid_registered();

            let manager = ToastManager::new(AUMID)
                .on_activated(None, move |_| {
                    activate_window_by_title(&title_for_callback);
                    let _ = tx_activated.send(());
                })
                .on_dismissed(move |_| {
                    let _ = tx_dismissed.send(());
                });

            let mut toast = Toast::new();
            toast.text1(&title);
            toast.text2(&message);

            if let Some(src) = &source {
                toast.text3(src);
            }

            if urgent {
                toast.scenario(winrt_toast_reborn::Scenario::Urgent);
            }

            manager.show(&toast)?;

            // Stay alive to handle click → window activation
            // Exits on click, dismissal, or 60s timeout
            let _ = rx.recv_timeout(Duration::from_secs(60));
        }

        Commands::Focus { title } => {
            if !activate_window_by_title(&title) {
                eprintln!("Window not found: {}", title);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
