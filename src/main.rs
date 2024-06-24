use std::{fs, sync::{Arc, Mutex}};

use actix_web::{cookie::time::Date, delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct AddInfo {
    title: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Todo {
    id: String,
    title: String,
}

impl Todo {
    fn new(id: String, title: String) -> Self {
        Todo { id, title }
    }
}

struct AppState {
    tasks: Mutex<Vec<Todo>>,
}

#[get("/todos")]
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(&*tasks)
}

#[get("/")]
async fn home() -> impl Responder {
    let contents = fs::read_to_string("index.html").unwrap();

    HttpResponse::Ok().body(contents)
}

#[get("/todos/{id}")]
async fn get_task(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id: String = path.into_inner();
    if let Some(task) = tasks.iter_mut().find(|x| x.id == id) {
        HttpResponse::Ok().json(task)
    }else {
        HttpResponse::NotFound().body(format!("{id} not found"))
    }
}

#[post("/todos")]
async fn add_task(data: web::Data<AppState>, req_body: web::Json<AddInfo>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id = Uuid::new_v4().to_string();
    let todo = Todo::new(id, req_body.title.to_string());
    tasks.push(todo.clone());
    HttpResponse::Ok().json(todo.clone())
}

#[put("/todos/{id}")]
async fn update_task(
    path: web::Path<String>,
    data: web::Data<AppState>,
    req_body: web::Json<AddInfo>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id: String = path.into_inner();
    if let Some(task) = tasks.iter_mut().find(|x| x.id == id) {
        task.title = req_body.title.clone();
    }
    HttpResponse::Ok().body(format!("{id} is modified"))
}

#[delete("/todos/{id}")]
async fn delete_task(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id: String = path.into_inner();
    let original_len = tasks.len();
    tasks.retain(|x| x.id != id);
    let new_len = tasks.len();

    if new_len < original_len {
        HttpResponse::Ok().body(format!("{id} is deleted"))
    } else {
        HttpResponse::NotFound().body(format!("{id} not found"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = Arc::new(AppState {
        tasks: Mutex::new(vec![
            Todo {
                id: "67e55044-10b1-426f-9247-bb680e5fe0c8".to_string(),
                title: "Learn jem".to_string(),
            }
        ]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_data.clone()))
            .service(get_tasks)
            .service(add_task)
            .service(update_task)
            .service(delete_task)
            .service(get_task)
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
