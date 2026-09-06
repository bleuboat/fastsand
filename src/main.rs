mod wiki;

use axum::{extract::Path, response::Html, routing::get, Router};
use wiki::Wiki;

async fn get_html(path: String) -> Html<String> {
    let wiki = Wiki::new(path);
    Html(wiki.render())
}

async fn start() -> Html<String> {
    let url = "start";
    get_html(url.to_owned()).await
}

async fn page(Path(path): Path<String>) -> Html<String> {
    get_html(path).await
}

#[tokio::main]
async fn main() {
    let mut app = Router::new();
    app = app.route("/", get(start));
    app = app.route("/{*path}", get(page));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
