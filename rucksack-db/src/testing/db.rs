use std::path::PathBuf;

use rucksack_lib::file;

pub fn tempdir() -> PathBuf {
    tempfile::tempdir().unwrap().path().to_owned()
}

pub fn tempfile() -> (PathBuf, String) {
    let mut file = tempdir();
    file.push("data");
    file.push("secrets");
    file.with_extension("db");
    let filename = file.display().to_string();
    let res = file::create_parents(filename.clone());
    assert!(res.is_ok());
    (file, filename)
}

pub fn tempbackups() -> (PathBuf, String) {
    let mut dir = tempdir();
    dir.push("backups");
    let res = file::create_dirs(dir.clone());
    assert!(res.is_ok());
    assert!(dir.exists());
    assert!(dir.is_dir());
    let dirname = dir.display().to_string();
    (dir, dirname)
}
