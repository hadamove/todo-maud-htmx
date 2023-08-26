use actix_files::Files as ActixFiles;
use actix_web::{get, post, web, App, HttpServer, Result as ActixResult};
use db::Database;
use maud::{html, Markup};

use crate::db::Todo;

mod db;
mod views;

#[get("/")]
pub async fn index(db: web::Data<Database>) -> ActixResult<Markup> {
    let todos = db.get_all().await.unwrap();

    let title = "Todo list";
    let content = html! {
        h1 { "Todo list" }
        (views::add_todo_view())
        (views::todos_view(todos))
    };

    Ok(views::base_view(title, content))
}

#[derive(serde::Deserialize)]
pub struct AddForm {
    text: String,
}

#[post("/add")]
async fn add(db: web::Data<Database>, form: web::Form<AddForm>) -> ActixResult<Markup> {
    let todo = db.insert(form.text.clone()).await.unwrap();

    Ok(views::todo_view(todo))
}

#[post("/toggle_done/{id}")]
async fn toggle_done(db: web::Data<Database>, path: web::Path<i64>) -> ActixResult<Markup> {
    let id = path.into_inner();
    let todo = db.get_by_id(id).await.unwrap();

    let todo = Todo {
        is_done: !todo.is_done,
        ..todo
    };
    let todo = db.update(todo).await.unwrap();

    println!("Toggled todo {} to {}", todo.id, todo.is_done);

    Ok(views::todo_view(todo))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database = Database::init_db().await.unwrap();

    println!("Listening on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(index)
            .service(add)
            .service(toggle_done)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
