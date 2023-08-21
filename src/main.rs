use actix_files::Files as ActixFiles;
use actix_web::web::Form;
use actix_web::{get, post, App, HttpRequest, HttpServer, Result as ActixResult};
use maud::{html, Markup};
use serde::Deserialize;

mod utils;

#[get("/")]
async fn index(req: HttpRequest) -> ActixResult<Markup> {
    let title = "actix-maud-htmx-h5bp";

    let content = html! {
        p { "Hello world! This is HTML5 Boilerplate." }

        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" {
            div {
                label { "What's your name? " }
                input type="text" name="name" value="" {}
            }
            button { "Submit" }
        }
    };

    Ok(utils::html_template(title, content))
}

#[derive(Deserialize)]
struct HelloForm {
    name: String,
}

#[post("/hello")]
async fn hello(user_input: Form<HelloForm>) -> ActixResult<Markup> {
    Ok(html! {
        #content {
            p { "Hello " (user_input.name) "! This is HTMX." }
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
