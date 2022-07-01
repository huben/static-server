use std::path::{ Path, PathBuf };
use std::fs::File;
use std::sync::Arc;

use walkdir::WalkDir;
use axum:: {
  body::{ Body, boxed },
  extract::Extension,
  http::{Request},
  response::{ IntoResponse, Response },
};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

use crate::model::{ FileInfo, FileTemplate, HtmlTemplate };

pub async fn fallback(Extension(root): Extension<Arc<String>>, req: Request<Body>) -> Response {
  let path = req.uri().path();
  let mut full_path = PathBuf::new();
  full_path.push(root.as_str());
  for seg in path.split("/") {
    full_path.push(seg)
  }
  let full_path = Path::new(&full_path);
  
  if let Ok(f) = File::open(full_path) {
    let metadata = f.metadata().unwrap();
    if !metadata.is_dir() {
      let res = ServeFile::new(full_path).oneshot(Request::new(Body::empty())).await.unwrap();
      return res.map(boxed);
    } 
  }
  let file_infos: Vec<FileInfo> = list(full_path);
  let ft = FileTemplate {
    files: file_infos
  };
  return HtmlTemplate(ft).into_response();
}

fn list(path: &Path) -> Vec<FileInfo> {
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