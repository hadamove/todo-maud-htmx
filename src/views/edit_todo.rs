use maud::{html, Markup};

use crate::db::Todo;

pub fn edit_todo_view(todo: Todo) -> Markup {
    let id = format!("todo-edit-{}", todo.id);
    html!(
        div class="my-1 p-2 bg-white rounded-md" id=(id) {
            form class="flex items-center gap-2"
                hx-post=(format!("/update/{}", todo.id))
                hx-target=(format!("#{}", id))
                hx-swap="outerHTML"
            {
                input class="flex-grow border border-gray-300 rounded-sm px-1" type="text" name="text" value=(todo.text) {}
                button class="ml-auto" type="submit" { "✅" }
            }
        }
    )
}
