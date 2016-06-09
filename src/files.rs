use std::env;
use std::io::Write;
use std::fs::{DirBuilder, File};
#[cfg(unix)]
use std::os::unix::fs::DirBuilderExt;
use std::path::{PathBuf, Path};

#[cfg(unix)]
fn create_folder(folder: &Path) {
    DirBuilder::new()
        .recursive(true)
        .mode(0o755)
        .create(folder)
        .expect("Couldn't create containing folder for file");
}

#[cfg(windows)]
fn create_folder(folder: &str) {
    DirBuilder::new()
        .recursive(true)
        .create(folder)
        .expect("Couldn't create containing folder for file");
}

fn build_file_path(hash: &str) -> PathBuf {
    // TODO: move this to server init
    let static_dir = env::var("STATIC_DIR").expect("STATIC_DIR must be set");

    let mut filepath = PathBuf::from(&static_dir);
    filepath.push(&hash[0..2]);
    filepath.push(&hash[2..4]);
    filepath.push(&hash);
    filepath.set_extension("pdf");
    filepath
}

pub fn save_file(hash: &str, bytes: &[u8]) {
    let filepath = build_file_path(hash);
    create_folder(filepath.parent().unwrap());

    let mut file = File::create(filepath).unwrap();
    file.write_all(bytes).unwrap();
}
