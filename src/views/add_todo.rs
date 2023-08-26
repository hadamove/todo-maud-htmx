use maud::{html, Markup};

pub(crate) fn add_todo_view() -> Markup {
    html! {
        div {
            form hx-post="/add" hx-target="#todo-list" hx-swap="afterbegin" {
                input type="text" name="text" placeholder="What needs to be done?" {}
                button type="submit" { "Add" }
            }
        }
    }
}
