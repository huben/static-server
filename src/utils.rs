use std::path::{ Path, PathBuf };

use walkdir::WalkDir;

use crate::model::{ FileInfo };

pub fn list(path: &Path) -> Vec<FileInfo> {
  let walkdir = WalkDir::new(path);
  let walkdir = walkdir.max_depth(1);
  let mut files: Vec<FileInfo> = Vec::new();
  for entry in walkdir {
    if let Ok(entry) = entry {
      if let Some(filename) = entry.path().file_name() {
        let filename = filename.to_owned().into_string().unwrap();
        let filename = &filename;
        if let Some(pathname) = path.file_name()  {
          if filename.eq(&pathname.to_owned().into_string().unwrap()) 
              && entry.file_type().is_dir() {
            continue
          }
        }
        let mut full_path = PathBuf::new();
        if path.is_relative() {
          full_path.push(path);
        }
        full_path.push(filename);
        files.push(FileInfo {
          name: filename.to_owned(),
          is_file: !entry.file_type().is_dir(),
          current_uri: Path::new(&full_path).to_str().unwrap().to_string(),
        });
      }
    }
  }
  files
}