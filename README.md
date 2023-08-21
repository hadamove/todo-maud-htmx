# Todo App

In this iteration, we will build a cliché todo app, which will allow us to learn the basics of reactive web apps using Actix Web and htmx.

The repository contains a skeleton of the app to make things easier for you, including a database access layer and a basic html/htmx template. You should not need to modify the database access layer, but you will need to add your own html/htmx templates, and endpoints, and write the application logic.

## Required Features & Grading

- List all items, add new items - **2 points**
- Mark items as completed - **1 point**
- Edit and delete items - **1 point**
- Clear all completed items - **1 point**

| <img src="demo.gif" width="420" /> |
| :--------------------------------: |
|     Demo of required features      |

**Remarks**:

- **All changes have to be persisted in the database so that they are visible after hitting refresh.**
- Feel free to make the app look how you want, as long as it's still easy to use (e.g. marking items as completed does not have to be a checkbox).
- You can use any database you want, the skeleton uses SQLite for simplicity.

## Running the skeleton

You will first need to create the database and run the migrations:

```
cargo install sqlx-cli
sqlx database create
sqlx migrate run --source db/migrations
```

The `sqlx` command line tool uses `DATABASE_URL` environment variable to connect to the database, the skeleton sets it to `sqlite://db/todo.sqlite3`, which will create an SQLite database file in the `db` directory.

> **Tip**: if you use VSCode as your IDE, you may use [SQLite Viewer](https://marketplace.visualstudio.com/items?itemName=qwtel.sqlite-viewer) extension to inspect the database directly from the editor.

After the database is set up, a simple `cargo run` should do the trick. If you happen to come across any problems, especially related to the database, don't hesitate to get in touch with us. We're here to help.

## Live Changes

If you want to see your changes right away as you work, [cargo-watch](https://crates.io/crates/cargo-watch) crate to recompile the app on every change:

```bash
cargo install cargo-watch
cargo watch -x run --ignore 'db/*'
```

> **Note**: `--ignore 'db/*'` is needed to avoid recompiling on every database change.

## Resources

- [Actix Web](https://actix.rs/docs/), especially [Extractors](https://actix.rs/docs/extractors/) and [Handlers](https://actix.rs/docs/handlers/) might be useful.
- [htmx](https://htmx.org/docs), the docs are quite long, reading through [introduction](https://htmx.org/docs/#introduction), [triggers](https://htmx.org/docs/#triggers), [targets](https://htmx.org/docs/#targets), and [swapping](https://htmx.org/docs/#swapping) should be enough to get you started.
- [Maud](https://maud.lambda.xyz/) is a macro-based html templating engine. The docs are short and simple, so reading everything is a good idea.
