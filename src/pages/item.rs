use maud::{html, Markup};

use crate::db::Item;

pub fn item_html(item: &Item) -> Markup {
    html! {
        div {
            h2 class="text-lg font-bold" { (item.name) }
            p class="text-gray-600" { (item.description) }
        }
    }
}
