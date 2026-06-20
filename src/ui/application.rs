use dioxus::prelude::*;
use crate::{handles::init_db, structs::AppState, ui::*};


#[component]
pub fn App() -> Element {
    use_context_provider(|| -> AppState {
        init_db().unwrap()
    });
    rsx! {
        //document::Link { rel: "icon", href: asset!("/assets/rand_icon.png") }
        Card { }
    }
}