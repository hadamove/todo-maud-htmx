use actix_files::Files as ActixFiles;
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Result as ActixResult};
use db::Database;
use maud::{html, Markup};
use serde::Deserialize;

mod db;
mod pages;
mod utils;

#[get("/")]
async fn index(db: web::Data<Database>, req: HttpRequest) -> ActixResult<Markup> {
    let title = "actix-maud-htmx-h5bp";

    let items = db.get_all().await.unwrap();

    let content = html! {
        p { "Hello world! This is HTML5 Boilerplate." }

        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" {
            div {
                label { "What's your name? " }
                input type="text" name="name" value="" {}
            }
            button { "Submit" }
        }

        ul {
            @for item in items {
                li {
                    {(pages::item::item_html(&item))}
                }
            }
        }
    };

    Ok(utils::html_template(title, content))
}

#[derive(Deserialize)]
struct HelloForm {
    name: String,
}

#[post("/hello")]
async fn hello(user_input: web::Form<HelloForm>) -> ActixResult<Markup> {
    Ok(html! {
        #content {
            p { "Hello " (user_input.name) "! This is HTMX." }
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database = Database::init_db().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(index)
            .service(hello)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
