use actix_web::{
    delete, get, middleware, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the index page!")
}

#[post("/create")]
async fn create() -> impl Responder {
    HttpResponse::Created().body("Resource created successfully!")
}

#[post("/create_user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    let new_user = user.into_inner();
    // Process the new user data (e.g., save it to a database)
    HttpResponse::Created().json(new_user)
}

#[put("/update")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("Resource updated successfully!")
}

#[delete("/delete")]
async fn delete() -> impl Responder {
    HttpResponse::NoContent().finish()
}

#[get("/user/{id}/{name}")]
async fn user_info(info: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = info.into_inner();
    HttpResponse::Ok().body(format!("User ID: {}, Name: {}", id, name))
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    email: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // Request logging middleware
            .wrap(middleware::Compress::default()) // Response compression middleware
            .service(index)
            .service(create)
            .service(update)
            .service(delete)
            .service(create_user)
            .service(user_info)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
