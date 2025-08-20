use axum::{
    Router,
    routing::{get, post},
};

async fn add_leaf() -> &'static str {
    "add_leaf"
}

async fn add_leaves() -> &'static str {
    "add_leaves"
}

async fn get_num_leaves() -> &'static str {
    "get_num_leaves"
}

async fn get_root() -> &'static str {
    "get_root"
}

async fn get_proof() -> &'static str {
    "get_proof"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/add-leaf", post(add_leaf))
        .route("/add-leaves", post(add_leaves))
        .route("/get-num-leaves", get(get_num_leaves))
        .route("/get-root", get(get_root))
        .route("/get-proof", get(get_proof));

    let ip = "127.0.0.1";
    let port = 3000;
    let addr = format!("{}:{}", ip, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server starting on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
