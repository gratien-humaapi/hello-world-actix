use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct AddInfo {
    title: String,
}

#[derive(Deserialize, Debug)]
struct Todo {
    id: usize,
    title: String,
}

impl Todo {
    fn new(title: String) -> Self {
        unsafe {
            let id = HELLO_WORLD.len();
            Todo { id, title }
        }
    }
}

static mut HELLO_WORLD: Vec<Todo> = Vec::new();

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/add-task")]
async fn add_task(req_body: web::Json<AddInfo>) -> impl Responder {
    unsafe {
        HELLO_WORLD.push(Todo::new(req_body.title.clone()));
        println!("body = {:?}", HELLO_WORLD);
    }

    HttpResponse::Ok().body(req_body.title.clone())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(add_task)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
