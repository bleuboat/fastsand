mod wiki;

use axum::{extract::Path, response::Html, routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use wiki::Wiki;

async fn get_html(path: String) -> Html<String> {
    let wiki = Wiki::new(path);
    Html(wiki.render())
}

async fn start() -> Html<String> {
    get_html("start".to_owned()).await
}

async fn page(Path(path): Path<String>) -> Html<String> {
    get_html(path).await
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut app = Router::new();
    app = app.route("/", get(start));
    app = app.route("/{*path}", get(page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("正在运行：http://localhost:3000");
    println!("按 Ctrl+C 退出");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
