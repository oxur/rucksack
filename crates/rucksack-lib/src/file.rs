use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs, io, path};

use anyhow::{anyhow, Context, Result};
use chrono::offset::Local;
use chrono::DateTime;
use path_clean::PathClean;

use crate::time;

const DATA_DIR: &str = "data";
const BACKUP_DIR: &str = "backups";
const DEFAULT_DB_NAME: &str = "secrets";
const DB_EXTENSION: &str = "db";

#[must_use = "path operation result must be checked"]
pub fn abs_path(path_name: String) -> io::Result<path::PathBuf> {
    let expanded = expanded_name(path_name);
    let path = path::Path::new(expanded.as_str());
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };
    absolute_path.clean();
    Ok(absolute_path)
}

pub fn backup_dir(project: &str) -> path::PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| path::PathBuf::from("."));
    path.push(project);
    path.push(BACKUP_DIR);
    path
}

pub fn config_dir(project: &str) -> path::PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| path::PathBuf::from("."));
    path.push(project);
    path
}

pub fn config_file(project: &str) -> String {
    let mut path = config_dir(project);
    path.push("config");
    path.set_extension("toml");
    path.to_str()
        .expect("config file path contains invalid UTF-8")
        .to_string()
}

#[must_use = "directory creation result must be checked"]
pub fn create_parents(path: String) -> Result<path::PathBuf> {
    // Make sure the path is created
    log::debug!(path = path.as_str(), operation = "create_parent"; "Attempting to create parent directory");
    let ap = abs_path(path.clone())?;
    let parent = ap
        .parent()
        .ok_or_else(|| anyhow!("path has no parent directory: {}", path))?
        .to_path_buf();
    log::debug!(path = parent.to_string_lossy().as_ref(), operation = "create_dir"; "Attempting to create directory");
    create_dirs(parent)?;
    Ok(ap)
}

#[must_use = "directory creation result must be checked"]
pub fn create_dirs(path: path::PathBuf) -> Result<path::PathBuf> {
    let path_name = path.display();
    match fs::create_dir_all(path.clone()) {
        Ok(_) => Ok(path),
        Err(e) => {
            let msg = "Could not create missing parent dirs for";
            log::error!(path = path_name.to_string().as_str(), error = e.to_string().as_str(), operation = "create_dir"; "{}", msg);
            Err(anyhow!("{} {} ({:})", msg, path_name, e))
        }
    }
}

pub fn data_dir(project: &str) -> path::PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| path::PathBuf::from("."));
    path.push(project);
    path.push(DATA_DIR);
    path
}

pub fn db_file(project: &str) -> String {
    let mut path = data_dir(project);
    path.push(DEFAULT_DB_NAME);
    path.set_extension(DB_EXTENSION);
    path.to_str()
        .expect("database file path contains invalid UTF-8")
        .to_string()
}

#[must_use = "file deletion result must be checked"]
pub fn delete(file_path: path::PathBuf) -> Result<()> {
    match fs::remove_file(file_path) {
        Ok(x) => {
            log::debug!(operation = "delete"; "Deleted file");
            Ok(x)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn dir_parent(dir: String) -> String {
    let mut parent: Vec<&str> = dir.split(std::path::MAIN_SEPARATOR).collect();
    parent.pop();
    parent.join(std::path::MAIN_SEPARATOR.to_string().as_str())
}

pub fn expanded_name(path_name: String) -> String {
    let expanded = shellexpand::tilde(path_name.as_str());
    expanded.to_string()
}

pub type Data = (String, String, String);
pub type Listing = Vec<Data>;

#[must_use = "directory listing result must be checked"]
pub fn files(dir: String) -> Result<Listing> {
    let mut f = Vec::<(String, String, String)>::new();
    for entry in fs::read_dir(dir)? {
        let dir = entry?;
        let metadata = dir.metadata()?;
        let created: DateTime<Local> = metadata.created()?.into();
        let file_name = dir
            .file_name()
            .to_str()
            .ok_or_else(|| anyhow!("file name contains invalid UTF-8"))?
            .to_owned();
        f.push((
            file_name,
            time::format_datetime(created),
            unix_mode::to_string(metadata.permissions().mode()),
        ));
    }
    Ok(f)
}

#[must_use = "file read result must be checked"]
pub fn read(file_name: String) -> Result<Vec<u8>> {
    let expanded = expanded_name(file_name.clone());
    log::debug!(file = expanded.as_str(), operation = "read"; "Reading file");
    fs::read(&expanded).with_context(|| format!("failed to read file: {}", file_name))
}

#[must_use = "file write result must be checked"]
pub fn write(data: Vec<u8>, path: String) -> Result<()> {
    let ap = create_parents(path.clone())?;
    // Then write the file
    log::debug!(file = ap.to_string_lossy().as_ref(), operation = "write"; "Writing file");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&ap)
        .with_context(|| format!("failed to open file for writing: {}", path))?;

    file.write_all(&data[..])
        .with_context(|| format!("failed to write data to file: {}", path))?;

    file.sync_all()
        .with_context(|| format!("failed to sync file to disk: {}", path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_expanded_name_no_tilde() {
        let path = "/usr/local/bin".to_string();
        assert_eq!(expanded_name(path.clone()), path);
    }

    #[test]
    fn test_expanded_name_with_tilde() {
        let path = "~/test".to_string();
        let expanded = expanded_name(path);
        assert!(!expanded.starts_with('~'));
        assert!(expanded.contains("test"));
    }

    #[test]
    fn test_expanded_name_empty() {
        let path = "".to_string();
        assert_eq!(expanded_name(path), "");
    }

    #[test]
    fn test_abs_path_absolute() {
        let path = "/tmp/test".to_string();
        let result = abs_path(path).unwrap();
        assert!(result.is_absolute());
    }

    #[test]
    fn test_abs_path_relative() {
        let path = "test".to_string();
        let result = abs_path(path).unwrap();
        assert!(result.is_absolute());
    }

    #[test]
    fn test_abs_path_with_tilde() {
        let path = "~/test".to_string();
        let result = abs_path(path).unwrap();
        assert!(result.is_absolute());
        assert!(!result.to_str().unwrap().contains('~'));
    }

    #[test]
    fn test_dir_parent_basic() {
        let dir = "/home/user/documents".to_string();
        let parent = dir_parent(dir);
        assert_eq!(parent, "/home/user");
    }

    #[test]
    fn test_dir_parent_root() {
        let dir = "/home".to_string();
        let parent = dir_parent(dir);
        assert_eq!(parent, "");
    }

    #[test]
    fn test_dir_parent_nested() {
        let dir = "/a/b/c/d/e".to_string();
        let parent = dir_parent(dir);
        assert_eq!(parent, "/a/b/c/d");
    }

    #[test]
    fn test_config_dir() {
        let project = "test_project";
        let path = config_dir(project);
        assert!(path.to_str().unwrap().contains(project));
    }

    #[test]
    fn test_config_file() {
        let project = "test_project";
        let file = config_file(project);
        assert!(file.contains(project));
        assert!(file.ends_with(".toml"));
        assert!(file.contains("config"));
    }

    #[test]
    fn test_data_dir() {
        let project = "test_project";
        let path = data_dir(project);
        let path_str = path.to_str().unwrap();
        assert!(path_str.contains(project));
        assert!(path_str.contains(DATA_DIR));
    }

    #[test]
    fn test_backup_dir() {
        let project = "test_project";
        let path = backup_dir(project);
        let path_str = path.to_str().unwrap();
        assert!(path_str.contains(project));
        assert!(path_str.contains(BACKUP_DIR));
    }

    #[test]
    fn test_db_file() {
        let project = "test_project";
        let file = db_file(project);
        assert!(file.contains(project));
        assert!(file.contains(DEFAULT_DB_NAME));
        assert!(file.ends_with(&format!(".{}", DB_EXTENSION)));
    }

    #[test]
    fn test_write_and_read() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.txt");
        let data = b"Hello, World!".to_vec();

        let result = write(data.clone(), file_path.to_str().unwrap().to_string());
        assert!(result.is_ok());

        let read_data = read(file_path.to_str().unwrap().to_string()).unwrap();
        assert_eq!(read_data, data);
    }

    #[test]
    fn test_write_empty_data() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("empty.txt");
        let data = Vec::new();

        let result = write(data.clone(), file_path.to_str().unwrap().to_string());
        assert!(result.is_ok());

        let read_data = read(file_path.to_str().unwrap().to_string()).unwrap();
        assert_eq!(read_data, data);
    }

    #[test]
    fn test_write_large_data() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("large.bin");
        let data = vec![42u8; 10000];

        let result = write(data.clone(), file_path.to_str().unwrap().to_string());
        assert!(result.is_ok());

        let read_data = read(file_path.to_str().unwrap().to_string()).unwrap();
        assert_eq!(read_data, data);
    }

    #[test]
    fn test_write_creates_parent_dirs() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("nested/dirs/file.txt");
        let data = b"test".to_vec();

        let result = write(data.clone(), file_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn test_read_nonexistent() {
        let result = read("/nonexistent/file/path.txt".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_create_dirs() {
        let dir = TempDir::new().unwrap();
        let new_dir = dir.path().join("new/nested/dirs");

        let result = create_dirs(new_dir.clone());
        assert!(result.is_ok());
        assert!(new_dir.exists());
    }

    #[test]
    fn test_create_dirs_already_exists() {
        let dir = TempDir::new().unwrap();
        let existing_dir = dir.path().to_path_buf();

        let result = create_dirs(existing_dir.clone());
        assert!(result.is_ok());
        assert!(existing_dir.exists());
    }

    #[test]
    fn test_create_parents() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("nested/file.txt");

        let result = create_parents(file_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
        assert!(dir.path().join("nested").exists());
    }

    #[test]
    fn test_delete_existing_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("delete_me.txt");
        fs::write(&file_path, b"test").unwrap();

        assert!(file_path.exists());
        let result = delete(file_path.clone());
        assert!(result.is_ok());
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_nonexistent() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("nonexistent.txt");

        let result = delete(file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_files_empty_dir() {
        let dir = TempDir::new().unwrap();
        let result = files(dir.path().to_str().unwrap().to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_files_with_contents() {
        let dir = TempDir::new().unwrap();
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");
        fs::write(&file1, b"test1").unwrap();
        fs::write(&file2, b"test2").unwrap();

        let result = files(dir.path().to_str().unwrap().to_string()).unwrap();
        assert_eq!(result.len(), 2);

        // Check that filenames are present
        let names: Vec<String> = result.iter().map(|(name, _, _)| name.clone()).collect();
        assert!(names.contains(&"file1.txt".to_string()));
        assert!(names.contains(&"file2.txt".to_string()));
    }

    #[test]
    fn test_files_nonexistent_dir() {
        let result = files("/nonexistent/directory".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_files_returns_metadata() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, b"test").unwrap();

        let result = files(dir.path().to_str().unwrap().to_string()).unwrap();
        assert_eq!(result.len(), 1);

        let (name, timestamp, permissions) = &result[0];
        assert_eq!(name, "file.txt");
        assert!(!timestamp.is_empty(), "Timestamp should not be empty");
        assert!(!permissions.is_empty(), "Permissions should not be empty");
    }

    #[test]
    fn test_write_overwrites_existing() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("overwrite.txt");
        let path_str = file_path.to_str().unwrap().to_string();

        // Write initial data
        let data1 = b"first".to_vec();
        write(data1, path_str.clone()).unwrap();

        // Overwrite with new data
        let data2 = b"second".to_vec();
        write(data2.clone(), path_str.clone()).unwrap();

        // Verify new data
        let read_data = read(path_str).unwrap();
        assert_eq!(read_data, data2);
    }

    #[test]
    fn test_constants() {
        assert_eq!(DATA_DIR, "data");
        assert_eq!(BACKUP_DIR, "backups");
        assert_eq!(DEFAULT_DB_NAME, "secrets");
        assert_eq!(DB_EXTENSION, "db");
    }
}
