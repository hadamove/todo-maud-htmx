use maud::{html, Markup};

pub fn clear_view() -> Markup {
    html!(
        div class="flex justify-end"{
            div class="py-1 px-2 bg-white rounded-md" {
                button hx-post="/clear" hx-target="#todo-list" hx-swap="outerHTML"
                {
                    small { "🔥 Clear done" }
                }
            }
        }
    )
}
