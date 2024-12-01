use axum::{routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use http::header::HeaderValue;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let todo_list: Arc<Mutex<Vec<TodoItem>>> = Arc::new(Mutex::new(Vec::new()));


    // Build the router
    let app = Router::new()
        .nest("/api", routes(todo_list.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(Extension(todo_list));

    // Define the server address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn routes(todo_list: Arc<Mutex<Vec<TodoItem>>>) -> Router {
    Router::new()
        .route("/todos", get(get_todos).post(add_todo))
        .layer(Extension(todo_list))
}

async fn get_todos(Extension(todo_list): Extension<Arc<Mutex<Vec<TodoItem>>>>) -> Json<Vec<TodoItem>> {
    let todo_list = todo_list.lock().unwrap();
    Json(todo_list.clone())
}

async fn add_todo(
    Extension(todo_list): Extension<Arc<Mutex<Vec<TodoItem>>>>,
    Json(todo): Json<PostTodoItem>
) -> Json<TodoItem> {
    let mut todo_list = todo_list.lock().unwrap();

    let new_todo = TodoItem {
        id: todo_list.len() as u64 + 1,
        title: todo.title,
        completed: todo.completed,
    };

    todo_list.push(new_todo.clone());

    Json(new_todo)
}


#[derive(Serialize, Deserialize, Clone)]
struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct PostTodoItem {
    title: String,
    completed: bool,
}