use std::cell::RefCell;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::{
  body::{boxed, Body},
  extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    Extension,
  },
  http::Request,
  response::{IntoResponse, Response},
};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

use crate::model::{ChatSession, FileInfo, FileTemplate, HtmlTemplate};
use crate::utils::list;
use crate::SOCKET_VEC;

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
      let res = ServeFile::new(full_path)
        .oneshot(Request::new(Body::empty()))
        .await
        .unwrap();
      return res.map(boxed);
    }
  }
  let file_infos: Vec<FileInfo> = list(full_path);
  let ft = FileTemplate { files: file_infos };
  return HtmlTemplate(ft).into_response();
}

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
  println!("connected");
  ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
  if let Some(msg) = socket.recv().await {
    if let Ok(msg) = msg {
      match msg {
        Message::Text(t) => {
          println!("client send str: {:?}", t);
        }
        Message::Binary(_) => {
          println!("client send binary data");
        }
        Message::Ping(_) => {
          println!("socket ping");
        }
        Message::Pong(_) => {
          println!("socket pong");
        }
        Message::Close(_) => {
          // delete chatsession
          println!("client disconnected");
          return;
        }
      }
    } else {
      println!("client disconnected");
      // delete chatsession
      return;
    }
  }

  SOCKET_VEC
    .lock()
    .unwrap()
    .push(RefCell::new(ChatSession { socket }));
}
