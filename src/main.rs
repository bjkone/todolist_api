use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{delete, get, post}};
use serde::{Serialize, Deserialize};
use std::{sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {
    println!("Threads Tokio : {}", std::thread::available_parallelism().unwrap());
    let app_state = AppState {
        taches: Arc::new(Mutex::new(Vec::new())),
    };
    let app = Router::new()
        .route("/taches", get(get_taches).post(creer_tache))
        .route("/taches/:nom", delete(supprimer_tache))
        .with_state(app_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Serveur démarré sur http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

    
}

async fn get_taches(State(state): State<AppState>) -> Json<Vec<Tache>> {
    let taches = state.taches.lock().unwrap();
    Json(taches.clone())
}

async fn creer_tache(State(state): State<AppState>, Json(tache): Json<Tache>) -> StatusCode {
    state.taches.lock().unwrap().push(tache);
    StatusCode::CREATED
}

async fn supprimer_tache(State(state): State<AppState>, Path(nom): Path<String>) -> StatusCode {
    let mut taches = state.taches.lock().unwrap();

    let existe = taches.iter().any(|t| t.name == nom);

    if !existe {
        return StatusCode::NOT_FOUND;
    }

    taches.retain(|t| t.name != nom);
    StatusCode::NO_CONTENT
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Tache {
    name: String,
    detail: String,
    priorite: Priorite,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum Priorite {
    Bas,
    Moyen,
    Haut,
}

#[derive(Clone)]
struct AppState {
    taches: Arc<Mutex<Vec<Tache>>>,
}