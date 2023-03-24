use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut confirm = false;
    let mut install = false;

    if args.len() < 2 {
        println!("Usage: {} <path> [--confirm] [--install]", args[0]);
        return;
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        println!("{} does not exist", path.display());
        return;
    }

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--confirm" => confirm = true,
            "--install" => install = true,
            _ => (),
        }
    }

    if path.is_file() {
        if confirm {
            print!("Are you sure you want to delete {}? [y/N] ", path.display());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if !input.trim().eq_ignore_ascii_case("y") {
                return;
            }
        }
        match fs::remove_file(path) {
            Ok(_) => println!("Deleted file {}", path.display()),
            Err(e) => println!("Error deleting file {}: {}", path.display(), e),
        }
    } else if path.is_dir() {
        if confirm {
            print!(
                "Are you sure you want to delete {} and all its contents? [y/N] ",
                path.display()
            );
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if !input.trim().eq_ignore_ascii_case("y") {
                return;
            }
        }
        match fs::remove_dir_all(path) {
            Ok(_) => println!("Deleted folder {}", path.display()),
            Err(e) => println!("Error deleting folder {}: {}", path.display(), e),
        }
    } else {
        println!("{} is not a file or folder", path.display());
    }

    if install {

        #[cfg(target_os = "windows")]
        fn install() -> io::Result<()> {
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;
            use winreg::{enums, RegKey};

            let exe_path = env::current_exe()?;
            let exe_path_str = exe_path.to_str().unwrap();

            let hklm = RegKey::predef(enums::HKEY_CLASSES_ROOT);
            let key = hklm.create_subkey("Directory\\Background\\shell\\Delete with Rust")?;
            key.set_value("", &OsString::from("Delete with Rust"))?;
            let command_key = key.create_subkey("command")?;
            command_key.set_value("", &OsString::from(format!("\"{}\" \"%V\"", exe_path_str)))?;

            Ok(())
        }

        #[cfg(target_os = "linux")]
        fn install() -> io::Result<()> {
            use std::fs::{self, File};
            use std::io::{self, Write};
            use std::path::{Path, PathBuf};

            let exe_path = env::current_exe()?;
            let exe_path_str = exe_path.to_str().unwrap();

            // For KDE Plasma
            let kde_plasma_dir = PathBuf::from("/usr/share/kservices5/ServiceMenus/");
            if kde_plasma_dir.exists() {
                let desktop_file_path = kde_plasma_dir.join("delete-with-rust.desktop");
                let mut desktop_file = File::create(&desktop_file_path)?;
                write!(
                    desktop_file,
                    r#"[Desktop Entry]
Type=Service
ServiceTypes=KonqPopupMenu/Plugin
Actions=delete_with_rust;

[Desktop Action delete_with_rust]
Name=Delete with Rust
Exec={}
Icon=<path-to-icon>
"#,
                    exe_path_str
                )?;
                desktop_file.flush()?;
                println!(
                    "Program installed as a shortcut to the right-click context menu on KDE Plasma"
                );
            }

            // For GNOME
            let gnome_dir = PathBuf::from("/usr/share/applications/");
            if gnome_dir.exists() {
                let desktop_file_path = gnome_dir.join("delete-with-rust.desktop");
                let mut desktop_file = File::create(&desktop_file_path)?;
                write!(
                    desktop_file,
                    r#"[Desktop Entry]
Type=Application
Name=Delete with Rust
Exec={}
NoDisplay=true
Icon=<path-to-icon>
"#,
                    exe_path_str
                )?;
                desktop_file.flush()?;
                println!(
                    "Program installed as a shortcut to the right-click context menu on GNOME"
                );
                let directory_file_path =
                    PathBuf::from("/usr/share/desktop-directories/delete-with-rust.directory");
                let mut directory_file = File::create(&directory_file_path)?;
                write!(
                    directory_file,
                    r#"[Desktop Entry]
Type=Directory
Name=Delete with Rust
Icon=<path-to-icon>
"#,
                )?;
                directory_file.flush()?;
            }

            Ok(())
        }

        #[cfg(target_os = "macos")]
        fn install() -> io::Result<()> {
            use std::process::Command;

            let exe_path = env::current_exe()?;
            let exe_path_str = exe_path.to_str().unwrap();

            Command::new("defaults")
                .args(&[
                    "write",
                    "com.apple.desktopservices",
                    "ContextMenuItems",
                    "-array-add",
                    format(
                        r#"'
                {{
                    Command = "{0}";
                    Name = "Delete with Rust";
                    Icon = "<path-to-icon>";
                }}
                '"#,
                        exe_path_str,
                    )
                    .as_str(),
                ])
                .output()
                .expect("failed to execute process");

            Ok(())
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os="macos")))]
        fn install() -> io::Result<()> {
            println!("This feature is only available on Windows, Linux and macOS");
            Ok(())
        }

        if install().is_ok() {
            println!("Program installed as a shortcut to the right-click context menu");
        }
    }
}
