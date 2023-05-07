use std::net::SocketAddr;
use axum::{
    response::Html,
    routing::get,
    Router,
};

struct Element {
    name: String,
    content: String,
    attributes: Vec<String>,
    children: Vec<Element>,
}

fn elem(name: String, content: String, attributes: Vec<String>, children: Vec<Element>) -> Element {
    Element {
        name,
        content,
        attributes,
        children,
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut attributes = String::new();
        for attribute in &self.attributes {
            attributes.push_str(&attribute);
            attributes.push(' ');
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
    Html(elem(
        "html".to_string(),
        "".to_string(),
        vec![
            "lang=\"en\"".to_string(),
        ],
        vec![
            elem(
                "head".to_string(),
                "".to_string(),
                vec![],
                vec![
                    elem(
                        "meta".to_string(),
                        "".to_string(),
                        vec![
                            "charset=\"UTF-8\"".to_string(),
                        ],
                        vec![],
                    ),
                    elem(
                        "meta".to_string(),
                        "".to_string(),
                        vec![
                            "name=\"viewport\"".to_string(),
                            "content=\"width=device-width, initial-scale=1.0\"".to_string(),
                        ],
                        vec![],
                    ),
                    elem(
                        "title".to_string(),
                        "Document".to_string(),
                        vec![],
                        vec![],
                    ),
                ],
            ),
            elem(
                "body".to_string(),
                "".to_string(),
                vec![],
                vec![
                    elem(
                        "h1".to_string(),
                        "Hello, world!".to_string(),
                        vec![],
                        vec![],
                    ),
                ],
            ),
        ],
    ).to_string())
}