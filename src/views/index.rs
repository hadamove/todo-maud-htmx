use maud::{html, Markup, DOCTYPE};

pub fn index_view(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html class="no-js" lang="en" {
            head {
                meta charset="utf-8";
                title { (title) }
            }
            body {
                (content)
                script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" crossorigin="anonymous" {}
                script src="https://cdn.tailwindcss.com" {}
            }
        }
    }
}
