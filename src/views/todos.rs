use maud::{html, Markup};

use crate::db::Todo;

pub fn todo_view(todo: Todo) -> Markup {
    let id = format!("todo-{}", todo.id);
    let is_checked = todo.is_done.then(|| 1);

    html! {
        div class="flex gap-1 my-1 p-2 items-center bg-white rounded-md" id=(id) {

            input type="checkbox" checked=[is_checked]
                hx-post=(format!("/toggle_done/{}", todo.id))
                hx-target=(format!("#{}", id))
                hx-swap="outerHTML" {}

            h2 class=(if todo.is_done { "line-through truncate" } else { "truncate" }) {
                (todo.text)
            }

            button class="ml-auto"
                hx-delete=(format!("/delete/{}", todo.id))
                hx-target=(format!("#{}", id))
                hx-trigger="click"
                hx-swap="delete" {
                "🗑️"
            }

            button class="ml-2"
                hx-get=(format!("/start_edit/{}", todo.id))
                hx-target=(format!("#{}", id))
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
        #todo-list {
            @for todo in todos {
                (todo_view(todo))
            }
        }
    }
}
