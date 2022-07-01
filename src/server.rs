use std::net::SocketAddr;
use std::str::FromStr;

use axum::{
	routing::{ get },
  Router,
};

use crate::handler;

#[tokio::main]
pub async fn run(host: &str, port: u16) {
  let app = Router::new()
      .route("/healthz2", get(|| async { "ok" }))
      .route("/healthz", get(|| async { "ok" }))
      .fallback(get(handler::fallback));
  
  let ip_addr = std::net::IpAddr::from_str(host).unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
  let addr = SocketAddr::from((ip_addr, port));
  
  println!("listening on {}", addr);

  axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
