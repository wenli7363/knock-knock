use clap::{Parser, Subcommand};
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
}

fn get_terminal_title() -> Option<String> {
    let mut buf = [0u16; 512];
    let len = unsafe { windows_sys::Win32::System::Console::GetConsoleTitleW(buf.as_mut_ptr(), buf.len() as u32) };
    if len == 0 {
        return None;
    }
    String::from_utf16(&buf[..len as usize]).ok()
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

            let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

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
        }
    }

    Ok(())
}
