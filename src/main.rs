use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
mod context_menu {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winreg::enums::{HKEY_CLASSES_ROOT, KEY_WRITE};
    use winreg::RegKey;

    pub fn add() {
        let path = "Directory\\Background\\shell\\delfast";
        let key = RegKey::predef(HKEY_CLASSES_ROOT)
            .create_subkey(path)
            .unwrap_or_default();
        key.set_value("", &"delfast").unwrap();
        let command_key = key.create_subkey("command").unwrap();
        command_key
            .set_value("", &format!("delfast.exe %1"))
            .unwrap();
    }

    pub fn is_installed() -> bool {
        let path = "Directory\\Background\\shell\\delfast";
        RegKey::predef(HKEY_CLASSES_ROOT).open_subkey(path).is_ok()
    }
}

#[cfg(target_os = "macos")]
mod context_menu {
    use cocoa::appkit::{NSStatusBar, NSStatusItem};
    use cocoa::base::{id, nil};
    use cocoa::foundation::{NSAutoreleasePool, NSString};

    pub fn add() {
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        let bar = NSStatusBar::systemStatusBar();
        let item = bar.statusItemWithLength(30.0);
        item.setTitle(NSString::alloc(nil).init_str("delfast"));
        item.setAction("open:".to_owned());
        drop(pool);
    }

    pub fn is_installed() -> bool {
        // TODO: Implement check for context menu installation on MacOS
        false
    }
}

#[cfg(target_os = "linux")]
mod context_menu {
    use std::fs;
    use std::path::Path;
    use xdg::BaseDirectories;

    pub fn add() {
        let xdg_dirs = BaseDirectories::new().unwrap();
        let path = xdg_dirs
            .place_data_file("delfast/delfast.desktop")
            .unwrap_or_else(|_| {
                panic!("Failed to create data file for delfast");
            });
        let menu_entry = "[Desktop Entry]
Name=delfast
Exec=delfast %f
Type=Application
NoDisplay=true
Categories=Utility;";
        fs::write(path, menu_entry).unwrap_or_else(|_| {
            panic!("Failed to write data file for delfast");
        });
    }

    pub fn is_installed() -> bool {
        let xdg_dirs = BaseDirectories::new().unwrap();
        let path = xdg_dirs
            .place_data_file("applications/delfast.desktop")
            .unwrap_or_default();
        Path::new(&path).exists()
    }
}

#[derive(Parser, Debug)]
#[command(name = "delfast", author, version, about, long_about = None)]
struct Args {
    /// Show confirmation prompt before deleting
    /// Show confirmation prompt before deleting
    #[arg(short, long, default_value = "false")]
    confirm: bool,

    /// Path to the folder to be deleted (relative or absolute)
    #[arg(parse(from_os_str))]
    path: PathBuf,
}
// Formatting strings
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

fn main() {
    // Get the command line arguments and the current working directory
    let args = Args::parse();
    let current_working_directory = env::current_dir().unwrap();
    if args.install_context_menu {
        // Check if context menu is installed
        let context_menu_installed = match std::env::consts::OS {
            "windows" => context_menu::is_installed(),
            "macos" => context_menu::is_installed(),
            "linux" => context_menu::is_installed(),
            _ => false,
        };

        // If context menu is already installed, exit the program
        if context_menu_installed {
            println!("{}Context menu is already installed!{}", GREEN, RESET);
            std::process::exit(0);
        }

        // Install the context menu
        match std::env::consts::OS {
            "windows" => context_menu::add(),
            "macos" => context_menu::add(),
            "linux" => context_menu::add(),
            // If the operating system is not supported, exit the program with an error message
            _ => {
                eprintln!(
                    "{}{}Error : [{}]",
                    RED, BOLD, "Operating system not supported"
                );
                std::process::exit(1);
            }
        }

        // Write a message to the console to indicate that the context menu was installed successfully
        println!("{}Context menu installed successfully!{}", GREEN, RESET);
        std::process::exit(0);
    }

    // Print the header information
    print_header(&current_working_directory, &args.path);

    // Show confirmation prompt
    if args.confirm {
        // If the user does not enter 'y' or 'Y', exit the program
        if !confirm_prompt() {
            println!("{}{}Exiting...{}", RESET, RED, RESET);
            print_fat_line(GREEN);
            std::process::exit(0);
        }
    }

    // Delete the folder
    print_line(GREEN);
    delete_folder(&args.path);
    print_fat_line(GREEN);
}

fn delete_folder(path: &PathBuf) {
    // Use pattern matching to handle the error
    match fs::remove_dir_all(path) {
        Ok(_) => println!("{}Deleted successfully!", GREEN),
        Err(e) => eprintln!("{}{}Error : [{}]", RED, BOLD, e),
    }
}

fn confirm_prompt() -> bool {
    print_line(GREEN);
    println!(
        "{}{}Are you sure you want to delete this folder? [y/n] {}{}",
        RED, BOLD, RESET, RESET
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // If the user enters 'y' or 'Y', return true
    input.trim() == "y" || input.trim() == "Y"
}

fn print_fat_line(color: &str) {
    println!(
        "{}{}===================================================={}",
        color, BOLD, RESET
    );
}

fn print_line(color: &str) {
    println!(
        "{}------------------------------------------------------{}",
        color, RESET
    );
}

fn print_header(current_working_directory: &PathBuf, path: &PathBuf) {
    print_fat_line(GREEN);
    println!("CWD is : {}{}", BLUE, &current_working_directory.display());
    print_line(GREEN);
    println!("Folder to be deleted : {}{}", BLUE, &path.display());
}
