use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};
use askama::Template;
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    // Better panics
    color_backtrace::install();

    // Logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    // Routes
    let router = Router::new()
        .route("/", get(index))
        .route("/projects", get(projects))
        .route("/blog", get(blog))
        .layer(TraceLayer::new_for_http())
        .nest_service("/static", ServeDir::new("static/"));

    // Start server
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Starting server on http://{address}");
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

async fn index() -> impl IntoResponse {
    Index {}
}

#[derive(Template)]
#[template(path = "projects.html")]
struct Projects {}

async fn projects() -> impl IntoResponse {
    Projects {}
}

#[derive(Template)]
#[template(path = "blog.html")]
struct Blog {}

async fn blog() -> impl IntoResponse {
    Blog {}
}
