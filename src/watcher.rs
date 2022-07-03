use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::SOCKET_VEC;

pub async fn run(root: &str) {
  println!("watch {:?}", root);
  let (tx, rx) = channel();
  let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
  watcher.watch(root, RecursiveMode::Recursive).unwrap();
  loop {
    match rx.recv() {
      Ok(event) => {
        println!("{:?}", event);

        for session in SOCKET_VEC.lock().unwrap().iter() {
          if let Ok(()) = session.borrow_mut().send(String::from("refresh")).await {
            println!("send message success");
          } else {
            println!("send message error");
          }
        }
      }
      Err(e) => println!("watch error: {:?}", e),
    }
  }
}
