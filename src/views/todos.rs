use maud::{html, Markup};

use crate::repository::Todo;

pub fn todo_view(todo: Todo) -> Markup {
    let is_checked = todo.is_done.then_some(1);

    html! {
        div
            class="flex gap-1 p-2 items-center bg-white rounded-md"
            hx-target="this"
        {
            input
                type="checkbox"
                checked=[is_checked]
                hx-post={ "/todos/" (todo.id) "/toggle_done" }
                hx-swap="outerHTML" {}

            h2 class=(if todo.is_done { "line-through truncate" } else { "truncate" }) {
                (todo.text)
            }

            button
                class="ml-auto"
                hx-delete={ "/todos/" (todo.id) }
                hx-trigger="click"
                hx-swap="delete" {
                "🗑️"
            }

            button
                class="ml-2"
                hx-get={ "/todos/" (todo.id) "/edit" }
                hx-trigger="click"
                hx-swap="outerHTML" {
                "✏️"
            }
        }
    }
}

pub fn todos_view(todos: Vec<Todo>) -> Markup {
    html! {
        #todo-list class="flex flex-col gap-2" {
            @for todo in todos {
                (todo_view(todo))
            }
        }
    }
}
