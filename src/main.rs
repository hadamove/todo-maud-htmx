use actix_files::Files as ActixFiles;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Result as ActixResult};
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
        div class="container mx-auto " {
            div class="max-w-md mx-auto my-10 bg-slate-100 p-5 rounded-lg"{
                h1 class="text-2xl font-bold text-center" {
                    "Items"
                }
                div class="p-5 flex flex-col gap-4" {
                    (views::todos_view(todos))
                    (views::clear_view())
                    (views::add_todo_view())
                }
            }
        }
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

    Ok(views::todo_view(todo))
}

#[delete("/delete/{id}")]
async fn delete(db: web::Data<Database>, path: web::Path<i64>) -> HttpResponse {
    let id = path.into_inner();

    db.delete(id).await.unwrap();

    HttpResponse::Ok().finish()
}

#[get("/start_edit/{id}")]
async fn start_edit(db: web::Data<Database>, path: web::Path<i64>) -> ActixResult<Markup> {
    let id = path.into_inner();
    let todo = db.get_by_id(id).await.unwrap();

    Ok(views::edit_todo_view(todo))
}

#[derive(serde::Deserialize)]
pub struct EditForm {
    text: String,
}

#[post("/update/{id}")]
async fn update(
    db: web::Data<Database>,
    path: web::Path<i64>,
    form: web::Form<EditForm>,
) -> ActixResult<Markup> {
    let id = path.into_inner();
    let todo = db.get_by_id(id).await.unwrap();

    let todo = Todo {
        text: form.text.clone(),
        ..todo
    };
    let todo = db.update(todo).await.unwrap();

    Ok(views::todo_view(todo))
}

#[post("/clear")]
async fn clear(db: web::Data<Database>) -> ActixResult<Markup> {
    db.delete_all_done().await.unwrap();

    let todos = db.get_all().await.unwrap();

    Ok(views::todos_view(todos))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database = Database::init_db().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(index)
            .service(add)
            .service(toggle_done)
            .service(delete)
            .service(start_edit)
            .service(update)
            .service(clear)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
