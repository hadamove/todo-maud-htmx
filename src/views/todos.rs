use maud::{html, Markup};

use crate::db::Todo;

pub(crate) fn todo_view(todo: Todo) -> Markup {
    let id = format!("todo-{}", todo.id);
    let is_checked = todo.is_done.then(|| 1);
    let toggle_url = format!("/toggle_done/{}", todo.id);

    html! {
        div class="flex" id=(id) {
            input type="checkbox" checked=[is_checked]
                hx-post=(toggle_url)
                hx-target=(format!("#{}", id))
                hx-swap="outerHTML" {}

            p class=(if todo.is_done { "line-through" } else { "" }) {
                (todo.text)
            }
        }
    }
}

pub(crate) fn todos_view(todos: Vec<Todo>) -> Markup {
    html! {
        // This is equivalent to: <div id="todo-list">...</div>
        #todo-list {
            @for todo in todos {
                (todo_view(todo))
            }
        }
    }
}
