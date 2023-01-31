use clap::Parser;
use std::env;
use std::fs;

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

fn main() {
    // Get the command line arguments and the current working directory
    let args = Args::parse();
    let current_working_directory = env::current_dir().unwrap();
    let path = current_working_directory.join(&args.path);

    print_fat_line(GREEN);
    println!("CWD is : {}{}", BLUE, &current_working_directory.display());
    print_line(GREEN);
    println!("Folder to be deleted : {}{}", BLUE, &path.display());

    // show confirmation prompt
    if args.confirm {
        print_line(GREEN);
        println!(
            "{}{}{}{}",
            RED, BOLD, "Are you sure you want to delete this folder? [y/n] ", RESET
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // if the user does not enter 'y' or 'Y', exit the program
        if input.trim() != "y" && input.trim() != "Y" {
            println!("{}{}Exiting...{}", RESET, RED, RESET);
            print_fat_line(GREEN);
            // exit the program
            std::process::exit(0);
        }
    }

    print_line(GREEN);

    // Delete the folder
    // use pattern matching to handle the error
    match fs::remove_dir_all(path) {
        Ok(_) => println!("{}Deleted successfully!", GREEN),
        Err(e) => println!("{}{}Error : [{}]", RED, BOLD, e),
    }
    print_fat_line(GREEN);
}
