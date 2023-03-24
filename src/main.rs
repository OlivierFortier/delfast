use clap::Clap;
use std::path::Path;

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
        println!("Program installed as a shortcut to the right-click context menu on KDE Plasma");
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
        println!("Program installed as a shortcut to the right-click context menu on GNOME");
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

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn install() -> io::Result<()> {
    println!("This feature is only available on Windows, Linux and macOS");
    Ok(())
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Your Name <your.email@example.com>")]
struct Opts {
    /// Path to the file or folder to delete
    path: String,

    /// Shows a confirmation prompt before deleting
    #[clap(short, long)]
    confirm: bool,

    /// Shows the different options that this program offers
    #[clap(short, long)]
    help: bool,

    /// Installs the program as a shortcut to the right-click context menu
    #[clap(long)]
    install: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.help {
        println!("Usage: delete_with_rust [OPTIONS] <path>");
        println!("\nOptions:");
        println!("  -c, --confirm    Shows a confirmation prompt before deleting");
        println!("  -h, --help       Shows the different options that this program offers");
        println!(
            "      --install    Installs the program as a shortcut to the right-click context menu"
        );
        return;
    }

    if opts.install {
        if install().is_ok() {
            println!("Program installed as a shortcut to the right-click context menu");
        }
        return;
    }

    let path = &opts.path;
    let confirm = opts.confirm;

    if confirm {
        println!("Are you sure you want to delete {}? (y/n)", path);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return;
        }
    }

    let path = Path::new(path);
    if path.exists() {
        std::fs::remove_dir_all(path).expect("Failed to delete the folder");
        println!("Deleted {}", path.display());
    } else {
        println!("Path not found: {}", path.display());
    }
}
