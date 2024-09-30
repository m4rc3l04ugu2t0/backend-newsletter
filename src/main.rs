use std::{fs::File, io::BufReader};

use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    name: String,
    title: String,
    description: String,
    img: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostImages {
    ip_a: String,
    lsblk: String,
    cfdisk: String,
    pattions: String,
}

#[tokio::main]
async fn main() {
    // Configura o middleware CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // Permitir requisições de qualquer origem
        .allow_methods([Method::GET, Method::POST]) // Permitir apenas GET e POST
        .allow_headers(Any); // Permitir qualquer cabeçalho

    // build our application with a single route
    let app = Router::new()
        .route("/api/posts", get(get_post_titles))
        .route("/api/:id/image", get(get_post_images))
        .nest_service("/api/assets", ServeDir::new("src/assets"))
        .layer(cors); // Aplica o CORS como camada

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_post_titles() -> impl IntoResponse {
    // Carrega o arquivo JSON
    let file = File::open("src/config/posts_titles.json").expect("Arquivo JSON não encontrado");
    let reader = BufReader::new(file);

    // Desserializa o JSON em um vetor de Posts
    let posts: Vec<Post> = serde_json::from_reader(reader).expect("Erro ao ler o JSON");

    // Retorna os posts como resposta JSON
    (StatusCode::OK, Json(posts))
}

async fn get_post_images(Path(post_name): Path<String>) -> impl IntoResponse {
    // Carrega o arquivo JSON
    let file = File::open(format!("src/config/{}/post_media.json", post_name))
        .expect("Arquivo JSON não encontrado");
    let reader = BufReader::new(file);

    // Desserializa o JSON em um vetor de Posts
    let post_images: Vec<PostImages> = serde_json::from_reader(reader).expect("Erro ao ler o JSON");

    // Retorna os posts como resposta JSON
    (StatusCode::OK, Json(post_images))
}
