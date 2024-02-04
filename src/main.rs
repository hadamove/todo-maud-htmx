use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer};
use maud::{html, Markup};

use crate::error::ApiResult;
use crate::repository::{Repository, Todo};

mod error;
mod repository;
mod views;

#[get("/")]
pub async fn index(db: web::Data<Repository>) -> ApiResult<Markup> {
    let todos = db.get_all().await?;

    let title = "Todo list";
    let content = html! {
        div class="container mx-auto" {
            div class="max-w-md mx-auto my-10 bg-slate-100 p-5 rounded-lg"{
                h1 class="text-2xl font-bold text-center" {
                    "Items"
                }
                div class="p-5 flex flex-col gap-4" {
                    (views::todos_view(todos))
                    (views::delete_done_view())
                    (views::add_todo_view())
                }
            }
        }
    };

    Ok(views::index_view(title, content))
}

#[derive(serde::Deserialize)]
pub struct AddForm {
    text: String,
}

#[post("/todos")]
async fn add_new(db: web::Data<Repository>, form: web::Form<AddForm>) -> ApiResult<Markup> {
    let todo = db.insert(form.into_inner().text).await?;

    Ok(views::todo_view(todo))
}

#[post("/todos/{id}/toggle_done")]
async fn toggle_done(db: web::Data<Repository>, path: web::Path<i64>) -> ApiResult<Markup> {
    let id = path.into_inner();
    let todo = db.get_by_id(id).await?;

    let todo = Todo {
        is_done: !todo.is_done,
        ..todo
    };
    let todo = db.update(todo).await?;

    Ok(views::todo_view(todo))
}

#[get("/todos/{id}/edit")]
async fn start_edit(db: web::Data<Repository>, path: web::Path<i64>) -> ApiResult<Markup> {
    let id = path.into_inner();
    let todo = db.get_by_id(id).await?;

    Ok(views::edit_todo_view(todo))
}

#[derive(serde::Deserialize)]
pub struct UpdateForm {
    text: String,
}

#[patch("/todos/{id}")]
async fn update(
    db: web::Data<Repository>,
    path: web::Path<i64>,
    form: web::Form<UpdateForm>,
) -> ApiResult<Markup> {
    let id = path.into_inner();
    let todo = db.get_by_id(id).await?;

    let todo = Todo {
        text: form.into_inner().text,
        ..todo
    };
    let todo = db.update(todo).await?;

    Ok(views::todo_view(todo))
}

#[delete("/todos/{id}")]
async fn delete(db: web::Data<Repository>, path: web::Path<i64>) -> ApiResult<HttpResponse> {
    let id = path.into_inner();

    db.delete(id).await?;

    Ok(HttpResponse::Ok().finish())
}

// TODO: change to query
#[delete("/todos")]
async fn delete_all_done(db: web::Data<Repository>) -> ApiResult<Markup> {
    db.delete_all_done().await?;

    let todos = db.get_all().await?;

    Ok(views::todos_view(todos))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database = Repository::init().await;

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(index)
            .service(add_new)
            .service(toggle_done)
            .service(start_edit)
            .service(update)
            .service(delete)
            .service(delete_all_done)
            .service(actix_files::Files::new("/", "./src/static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
