use std::{fs, sync::{Arc, Mutex}};

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AddInfo {
    title: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Todo {
    id: usize,
    title: String,
}

impl Todo {
    fn new(id: usize, title: String) -> Self {
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
    path: web::Path<usize>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id: usize = path.into_inner();
    if let Some(task) = tasks.iter_mut().find(|x| x.id == id) {
        HttpResponse::Ok().json(task)
    }else {
        HttpResponse::NotFound().body(format!("{id} not found"))
    }
}

#[post("/todos")]
async fn add_task(data: web::Data<AppState>, req_body: web::Json<AddInfo>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id = tasks.len();
    let todo = Todo::new(id, req_body.title.to_string());
    tasks.push(todo.clone());
    HttpResponse::Ok().json(todo.clone())
}

#[put("/todos/{id}")]
async fn update_task(
    path: web::Path<usize>,
    data: web::Data<AppState>,
    req_body: web::Json<AddInfo>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id: usize = path.into_inner();
    if let Some(task) = tasks.iter_mut().find(|x| x.id == id) {
        task.title = req_body.title.clone();
    }
    HttpResponse::Ok().body(format!("{id} is modified"))
}

#[delete("/todos/{id}")]
async fn delete_task(
    path: web::Path<usize>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id: usize = path.into_inner();
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
                id: 0,
                title: "Learn jem".to_string(),
            },
            Todo {
                id: 1,
                title: "Learn rem".to_string(),
            },
            Todo {
                id: 2,
                title: "Learn rtt".to_string(),
            },
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
