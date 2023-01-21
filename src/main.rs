use std::fs;

fn main() {
    let path = r"C:\Users\Olivier\Documents\work\ConnectIt\SPB\spb-portail-maintenance";

    fs::remove_dir_all(path).unwrap();
}
