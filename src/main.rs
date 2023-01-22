use std::fs;
use std::env;

fn main() {
    // Get the command line arguments and the current working directory
    let args: Vec<_> = env::args().collect();
    let current_path = env::current_dir().unwrap();

    let green = "\x1b[32m";
    let blue = "\x1b[34m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!("{}{}====================================================", green, bold);
    println!("CWD is : {}{}", blue, &current_path.display());
    println!("{}------------------------------------------------------", green);

    // Get the path of the folder to be deleted
    let path = current_path.join(&args[1]);
    println!("Folder to be deleted : {}{}", blue, &path.display());
    println!("{}------------------------------------------------------", green);

    // Delete the folder
    fs::remove_dir_all(path).unwrap_or(println!("{}{}Error: Folder '{}' not found!", red, bold, &args[1]));
    println!("{}Operation performed successfully!", green);
    println!("===================================================={}", reset);
}
