use actix_files::Files as ActixFiles;
use actix_web::{get, post, web, App, HttpServer, Result as ActixResult};
use maud::{html, Markup};
use repository::Repository;

mod repository;
mod views;

// Feel free to delete or modify this route. This is just an example.
#[get("/")]
async fn index(repo: web::Data<Repository>) -> ActixResult<Markup> {
    let title = "Hello from htmx! 🦀";

    // This is an example of how to use the database.
    let todos = repo.get_all().await.unwrap();
    assert!(todos.is_empty(), "The database should be empty.");

    let content = html! {
        // This is equivalent to <div id="content">...</div>
        #content {
            p { "Hello world! This text will be replaced on request." }
        }

        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" {
            div {
                label { "What's your name? " }
                // You can use tailwind CSS to style your views.
                input class="border border-gray-500" type="text" name="name" value="" {}
            }
            button { "Submit" }
        }
    };

    Ok(views::base_view(title, content))
}

#[derive(serde::Deserialize)]
struct HelloForm {
    name: String,
}

#[post("/hello")]
async fn hello(user_input: web::Form<HelloForm>) -> ActixResult<Markup> {
    // You can use `src/views` to structure your views.
    Ok(views::hello_view(user_input.name.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database = Repository::try_init().await.unwrap();

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(index)
            .service(hello)
            // If you're going to add new routes, don't forget to add them here!
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
