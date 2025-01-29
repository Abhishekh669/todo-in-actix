mod api;
mod models;
mod repository;


use actix_cors::Cors;
use actix_web::{http, web::Data, App, HttpServer};
use api::todo_api::{create_todo, delete_todo, get_all_todos, get_todo, update_todo}; //import the handler here
use repository::mongodb_repo::MongoRepo;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://192.168.1.70:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
            )
            .service(create_todo)
            .service(get_all_todos)
            .service(update_todo)
            .service(delete_todo)
            .service(get_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}