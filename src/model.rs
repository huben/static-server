use askama::Template;
use axum:: {
  http::{ StatusCode },
  response::{ IntoResponse, Response, Html },
};

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