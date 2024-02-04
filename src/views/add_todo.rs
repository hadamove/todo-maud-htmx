use maud::{html, Markup};

pub fn add_todo_view() -> Markup {
    html! {
        div class="p-1 bg-white rounded-md" {
            form
                class="flex items-center gap-2"
                hx-post="/todos"
                hx-target="#todo-list"
                hx-swap="afterbegin"
                // Remove the value from the input after adding a new todo
                hx-on="htmx:afterRequest: this.reset();"
            {
                input class="flex-grow rounded-sm mx-1"
                    type="text"
                    name="text"
                    placeholder="What needs to be done?" {}

                button type="submit" class="ml-auto" { "➕" }
            }
        }
    }
}
