use crate::ui::*;

mod errors;
mod handles;
mod structs;
mod ui;

fn main() {
    dioxus::launch(app);
}
