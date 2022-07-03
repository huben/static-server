use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use axum::{
  extract::Extension,
	routing::{ get },
  Router,
  Server,
};

use crate::handler;

#[tokio::main]
pub async fn run(host: &str, port: u16, root: String) {
  
  let app = Router::new()
      .route("/healthz", get(|| async { "ok" }))
      .route("/ws", get(handler::ws_handler))
      .fallback(get(handler::fallback))
      .layer(Extension(Arc::new(root)));
  
  let ip_addr = std::net::IpAddr::from_str(host).unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
  let addr = SocketAddr::from((ip_addr, port));
  
  println!("listening on {}", addr);

  Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
