use std::{net::SocketAddr};
use axum::{
  response::Html,
  routing::get,
  Router,
};

struct Element {
  name: String,
  content: String,
  attributes: Vec<(String, String)>,
  children: Vec<Element>,
}

fn elem(name: &str, content: &str, attributes: Vec<(&str, &str)>, children: Vec<Element>) -> Element {
  Element {
    name: name.to_string(),
    content: content.to_string(),
    attributes: attributes.iter().map(|attr| {
      (attr.0.to_string(), attr.1.to_string())
    }).collect(),
    children,
  }
}

impl std::fmt::Display for Element {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut attributes = String::new();
    for attribute in &self.attributes {
        attributes.push_str(&format!("{}=\"{}\" ", attribute.0, attribute.1));
        //attributes.push(' ');
    }
    let mut children = String::new();
    for child in &self.children {
        children.push_str(&child.to_string());
    }
    write!(f, "<{} {}>{}{}</{}>", self.name, attributes, self.content, children, self.name)
  }
}

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(root));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> Html<String> {
  Html(elem("html", "", vec![ ("lang", "en") ], vec![
    elem("head", "", vec![], vec![
      elem("meta", "", vec![ ("charset", "UTF-8") ], vec![]),
      elem("title", "Document", vec![], vec![]),
    ]),
    elem(
      "body", "", vec![], vec![
        elem("h1", "Hello, world!", vec![], vec![]),
        elem("p", "Hello, world!", vec![], vec![]),
      ],
    ),
  ]).to_string())
}
