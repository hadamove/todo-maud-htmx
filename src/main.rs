use actix_web::{web, App, HttpServer};

use crate::repository::Repository;

mod error;
mod handlers;
mod repository;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database = Repository::init().await;

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(handlers::index)
            .service(handlers::add_new)
            .service(handlers::toggle_done)
            .service(handlers::start_edit)
            .service(handlers::update)
            .service(handlers::delete)
            .service(handlers::delete_all_done)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
