use maud::{html, Markup, DOCTYPE};

// Feel free to update this function to your needs.
// For styling, you can either put your own CSS in static/css/ or use Tailwind CSS.
pub fn base_view(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html class="no-js" lang="en" {
            head {
                meta charset="utf-8";
                title { (title) }
                link rel="stylesheet" href="css/main.css";
            }
            body {
                (content)
                script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" crossorigin="anonymous" {}
                script src="https://cdn.tailwindcss.com" {}
            }
        }
    }
}
