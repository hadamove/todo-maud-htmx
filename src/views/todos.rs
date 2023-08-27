use maud::{html, Markup};

use crate::repository::Todo;

pub fn todo_view(todo: Todo) -> Markup {
    let id = format!("todo-{}", todo.id);
    let is_checked = todo.is_done.then_some(1);

    html! {
        div class="flex gap-1 p-2 items-center bg-white rounded-md" id=(id) {

            // Conditional attribute, see https://maud.lambda.xyz/splices-toggles.html
            input type="checkbox" checked=[is_checked]
                // Concarnation can be done within the curly braces, see https://maud.lambda.xyz/splices-toggles.html
                hx-post={ "/toggle_done/" (todo.id) }
                hx-target={ "#"(id) }
                hx-swap="outerHTML" {}

            // There is probably a better way to do this
            h2 class=(if todo.is_done { "line-through truncate" } else { "truncate" }) {
                (todo.text)
            }

            button class="ml-auto"
                hx-delete={ "/delete/" (todo.id) }
                hx-target={ "#"(id) }
                hx-trigger="click"
                hx-swap="delete" {
                "🗑️"
            }

            button class="ml-2"
                hx-get={ "/start_edit/" (todo.id) }
                hx-target={ "#"(id) }
                hx-trigger="click"
                hx-swap="outerHTML" {
                "✏️"
            }
        }
    }
}

pub fn todos_view(todos: Vec<Todo>) -> Markup {
    html! {
        // This is equivalent to: <div id="todo-list">...</div>
        #todo-list class="flex flex-col gap-2" {
            // https://maud.lambda.xyz/control-structures.html
            @for todo in todos {
                (todo_view(todo))
            }
        }
    }
}
