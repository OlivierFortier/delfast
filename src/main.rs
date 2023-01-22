use std::fs;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let current_path = env::current_dir().unwrap();

    println!("============================");
    println!("CWD is : {}", &current_path.display());
    println!("============================");

    let path = current_path.join(&args[1]);
    println!("Folder to be deleted : {}", &path.display());
    fs::remove_dir_all(path).unwrap();
}
