use crate::ui::*;

mod errors;
mod handles;
mod structs;
mod ui;

use dioxus::prelude::*;
fn main() {
    #[cfg(feature = "desktop")] 
    {
        use dioxus::{LaunchBuilder, desktop::{Config, LogicalSize, WindowBuilder,}};
        let window = WindowBuilder::new()
            .with_title("Subject Randomizer")
            .with_min_inner_size(LogicalSize::new(500.0, 500.0))
            .with_resizable(true);
            
        let config = Config::new()
            .with_window(window);
            
        LaunchBuilder::new().with_cfg(config).launch(App);
    }
    #[cfg(feature = "mobile")]
    {
        dioxus::LaunchBuilder::new().launch(App)
    }
}

// Idiomatic helper: Encapsulates the "Raw Pixel" boilerplate required by the window manager
// fn get_app_icon() -> Option<Icon> {
//     let bytes = include_bytes!("../assets/rand_icon.png");
//     
//     
//         image::load_from_memory(bytes).ok()
//         .map(|img| {
//             let rgba = img.to_rgba8();
//             let (w, h) = rgba.dimensions();
//             Icon::from_rgba(rgba.into_raw(), w, h).ok()
//         })
//         .flatten()
//}
