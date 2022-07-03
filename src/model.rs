use askama::Template;
use axum:: {
  Error,
  extract::{
    ws::{Message, WebSocket},
  },
  http::{ StatusCode },
  
  response::{ IntoResponse, Response, Html },
};
use clap::Parser;

/// simple development tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// select host address to bind to
	#[clap(short, long, value_parser, default_value = "127.0.0.1")]
	pub host: String,
  /// select port to use
	#[clap(short, long, value_parser, default_value_t = 3030)]
	pub port: u16,
  /// PATH instead of server root
	#[clap(short, long, value_parser, default_value = ".")]
	pub root: String,
  /// launch browser 
	#[clap(short, long, action)]
	pub open: bool,
}

impl Args {
  pub fn new() -> Self {
    Args::parse()
  }
}

#[derive(Debug)]
pub struct FileInfo {
  pub name: String,
  pub is_file: bool,
  pub current_uri: String,
}

#[derive(Template)] 
#[template(path = "index.html")]
pub struct FileTemplate {
  pub files: Vec<FileInfo>
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where T: Template 
{
  fn into_response(self) -> Response {
    match self.0.render() {
      Ok(html) => Html(html).into_response(),
      Err(err) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("failed to render template: {}", err),
      ).into_response(),
    }
  }
} 

pub struct ChatSession<> {
  pub socket: WebSocket
}

impl ChatSession {
  pub async fn send(&mut self, s: String) -> Result<(), Error> {
    self.socket.send(Message::Text(s)).await
  }
}