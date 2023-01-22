use std::fs;
use std::env;

fn main() {
    // Get the command line arguments and the current working directory
    let args: Vec<_> = env::args().collect();
    let current_path = env::current_dir().unwrap();

    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!("{} {} ====================================================", green, bold);
    println!("CWD is : {}", &current_path.display());
    println!("------------------------------------------------------");

    // Get the path of the folder to be deleted
    let path = current_path.join(&args[1]);
    println!("Folder to be deleted : {}", &path.display());
    println!("==================================================== {}" , reset);

    // Delete the folder
    fs::remove_dir_all(path).unwrap();
}
