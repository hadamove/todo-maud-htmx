use maud::{html, Markup};

use crate::repository::Todo;

pub fn edit_todo_view(todo: Todo) -> Markup {
    html!(
        div
            class="my-1 p-2 bg-white rounded-md"
            hx-target="this"
        {
            form
                class="flex items-center gap-2"
                hx-patch={ "/todos/" (todo.id) }
                hx-swap="outerHTML"
            {
                input
                    class="flex-grow border border-gray-300 rounded-sm px-1"
                    type="text"
                    name="text"
                    value=(todo.text) {}

                button class="ml-auto" type="submit" { "✅" }
            }
        }
    )
}
