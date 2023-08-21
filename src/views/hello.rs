use maud::{html, Markup};

pub fn hello_view(name: String) -> Markup {
    html! {
        #content {
            p { "Hello " (name) "!" }
        }
    }
}
