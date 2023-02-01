use clap::Parser;
use std::env;
use std::fs;

// #[cfg(target_os = "windows")]
// mod windows {
//     use winreg::RegKey;
//     use winreg::enums::{HKEY_CLASSES_ROOT, KEY_WRITE};
//     use std::ffi::OsStr;
//     use std::os::windows::ffi::OsStrExt;

//     pub fn add_context_menu() {
//         let path = "Directory\\Background\\shell\\delfast";
//         let key = RegKey::predef(HKEY_CLASSES_ROOT)
//             .create_subkey(path)
//             .unwrap();
//         key.set_value("", &"delfast").unwrap();
//         let command_key = key.create_subkey("command").unwrap();
//         command_key.set_value("", &format!("delfast.exe %1")).unwrap();
//     }
// }

// #[cfg(target_os = "macos")]
// mod macos {
//     use cocoa::base::{id, nil};
//     use cocoa::appkit::{NSStatusItem, NSStatusBar};
//     use cocoa::foundation::{NSString};

//     pub fn add_context_menu() {
//         let bar = NSStatusBar::systemStatusBar();
//         let item = bar.statusItemWithLength(30.0);
//         item.setTitle(NSString::alloc(nil).init_str("delfast"));
//         item.setAction("open:".to_owned());
//     }
// }

// #[cfg(target_os = "linux")]
// mod linux {
//     use xdg::BaseDirectories;
//     use std::fs;
//     use std::path::Path;

//     pub fn add_context_menu() {
//         let xdg_dirs = BaseDirectories::new().unwrap();
//         let path = xdg_dirs.place_data_file("delfast/delfast.desktop").unwrap();
//         let menu_entry = "[Desktop Entry]
//         Name=delfast
//         Exec=delfast %f
//         Type=Application
//         NoDisplay=true
//         Categories=Utility;";
//         fs::write(path, menu_entry).unwrap();
//     }
// }
#[derive(Parser, Debug)]
#[command(name="delfast", author, version, about, long_about = None)]
struct Args {
    /// Show confirmation prompt before deleting
    #[arg(short, long, default_value_t = false)]
    confirm: bool,

    /// Path to the folder to be deleted (relative or absolute)
    path: String,
}

// Formatting strings
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

fn main() {

    // TODO: Add context menu for Windows, MacOS and Linux
    // #[cfg(target_os = "windows")]
    // windows::add_context_menu();

    // #[cfg(target_os = "macos")]
    // macos::add_context_menu();

    // #[cfg(target_os = "linux")]
    // linux::add_context_menu();

    // Get the command line arguments and the current working directory
    let args = Args::parse();
    let current_working_directory = env::current_dir().unwrap();
    let path = current_working_directory.join(&args.path);

    print_header(current_working_directory, &path);

    // show confirmation prompt
    if args.confirm {
        // if the user does not enter 'y' or 'Y', exit the program
        if !confirm_prompt() {
            println!("{}{}Exiting...{}", RESET, RED, RESET);
            print_fat_line(GREEN);
            // exit the program
            std::process::exit(0);
        }
    }

    print_line(GREEN);
    delete_folder(path);
    print_fat_line(GREEN);
}

fn delete_folder(path: std::path::PathBuf) {
    // use pattern matching to handle the error
    match fs::remove_dir_all(path) {
        Ok(_) => println!("{}Deleted successfully!", GREEN),
        Err(e) => println!("{}{}Error : [{}]", RED, BOLD, e),
    }
}

fn confirm_prompt() -> bool {
    print_line(GREEN);
    println!("{}{}Are you sure you want to delete this folder? [y/n] {}{}", RED, BOLD, RESET, RESET);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // if the user enters 'y' or 'Y', return true
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

fn print_header(current_working_directory: std::path::PathBuf, path: &std::path::PathBuf) {
    print_fat_line(GREEN);
    println!("CWD is : {}{}", BLUE, &current_working_directory.display());
    print_line(GREEN);
    println!("Folder to be deleted : {}{}", BLUE, &path.display());
}
