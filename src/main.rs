use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

mod merkle;
use merkle::{Merkle, hash_2_string};

type AppState = Arc<Mutex<Merkle>>;

async fn add_leaf(State(state): State<AppState>, Json(req): Json<String>) -> impl IntoResponse {
    let mut merkle = state.lock().unwrap();
    merkle.add_leaf(req.as_bytes());
    Json(())
}

async fn add_leaves(
    State(state): State<AppState>,
    Json(req): Json<Vec<String>>,
) -> impl IntoResponse {
    let mut merkle = state.lock().unwrap();
    for leaf in req {
        merkle.add_leaf(leaf.as_bytes());
    }
    Json(())
}

async fn get_num_leaves(State(state): State<AppState>) -> impl IntoResponse {
    let merkle = state.lock().unwrap();
    Json(merkle.get_num_leaves())
}

async fn get_root(State(state): State<AppState>) -> impl IntoResponse {
    let merkle = state.lock().unwrap();
    let root = merkle.get_root();
    Json(hash_2_string(&root))
}

async fn get_proof(State(state): State<AppState>, Json(req): Json<usize>) -> impl IntoResponse {
    let merkle = state.lock().unwrap();
    let (leaf_hash, proof) = merkle.get_proof(req);
    Json(serde_json::json!({
        "leaf_hash": hash_2_string(&leaf_hash),
        "proof": proof.iter().map(|h| hash_2_string(h)).collect::<Vec<_>>()
    }))
}

#[tokio::main]
async fn main() {
    // Parameters
    let depth = 32;
    let ip = "127.0.0.1";
    let port = 3000;
    let addr = format!("{}:{}", ip, port);

    let state = Arc::new(Mutex::new(Merkle::new(depth)));
    let app = Router::new()
        .route("/add-leaf", post(add_leaf))
        .route("/add-leaves", post(add_leaves))
        .route("/get-num-leaves", get(get_num_leaves))
        .route("/get-root", get(get_root))
        .route("/get-proof", get(get_proof))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server starting on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
