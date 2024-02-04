use maud::{html, Markup};

use crate::repository::Todo;

pub fn todo_view(todo: Todo) -> Markup {
    // TODO: replace with parent swap
    let element_id = format!("todo-{}", todo.id);
    let is_checked = todo.is_done.then_some(1);

    html! {
        div class="flex gap-1 p-2 items-center bg-white rounded-md" id=(element_id) {

            input
                type="checkbox"
                checked=[is_checked]
                hx-post={ "/todos/" (todo.id) "/toggle_done" }
                hx-target={ "#"(element_id) }
                hx-swap="outerHTML" {}

            h2 class=(if todo.is_done { "line-through truncate" } else { "truncate" }) {
                (todo.text)
            }

            button
                class="ml-auto"
                hx-delete={ "/todos/" (todo.id) }
                hx-target={ "#"(element_id) }
                hx-trigger="click"
                hx-swap="delete" {
                "🗑️"
            }

            button
                class="ml-2"
                hx-get={ "/todos/" (todo.id) "/edit" }
                hx-target={ "#"(element_id) }
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
