use std::time::Duration;
use dioxus::prelude::*;
use rand::seq::IndexedRandom;
use tokio::task::spawn_blocking;

use crate::handles::randomize::query_obj;
use crate::structs::AppState;
use crate::structs::module::RandObject;

#[component]
pub fn Card() -> Element {
    let r_obj = use_signal(|| RandObject::new());
    let is_animating = use_signal(|| false);
    let db = use_context::<AppState>();

    let mut topics_resource = use_resource(move || {
        let db = db.clone();
        async move {
            spawn_blocking(move || {
                let conn = db.pool.lock().map_err(|_| "DB Lock failed")?;
                query_obj(&conn).map_err(|e| e.to_string())
            })
            .await
            .map_err(|e| e.to_string())?
        }
    });

    let RandObject { 
        topic_name, 
        mod_id, 
        subj_name
    } = r_obj();

    let resource_read = topics_resource.read();
    
    match &*resource_read {
        Some(Ok(topics)) => {
            rsx! {
                document::Stylesheet { href: asset!("/assets/main.css")}
                div {  
                    class: "main-container",
                    div { 
                        class: "header",
                        "Subject Randomizer"
                    }
                    div {  
                        class: "circle-container",
                        div { 
                            class: if is_animating() { 
                                "circle animating" 
                            } else if mod_id == 0 {
                                "circle animate-morph"
                            } else { 
                                "circle" 
                            },
                            div { 
                                class: "obj-data",
                                if !is_animating() && mod_id == 0 {
                                    h2 { 
                                        id: "welcome-greeting", 
                                        class: "animate-pop-in",
                                        "Hello!" 
                                    }
                                    p { 
                                        id: "placeholder", 
                                        class: "animate-fade-in delay-1",
                                        "Ready to study? Press Randomize to pick a topic." 
                                    }
                                } else {
                                    div {
                                        class: "obj-data",
                                        if !is_animating() { 
                                            p { 
                                                id: "module", 
                                                class: "animate-fade-in",
                                                "Module {mod_id}" 
                                            }
                                        }
                                        p { 
                                            id: "topic",
                                            // Topic always shows, but we can animate its text change if needed
                                            "{topic_name}" 
                                        }
                                        if !is_animating() && !subj_name.is_empty() {
                                            p { 
                                                id: "subject", 
                                                class: "animate-pop-in delay-1",
                                                "{subj_name}"
                                            }
                                        }
                                    }
                                }
                             }
                        }
                    }
                    RandTopic { 
                        topic: r_obj, 
                        is_animating, 
                        id: mod_id,
                        available_topics: topics.clone() 
                    }
                }
            }
        },
        None => rsx! {
            div { class: "main-container", "Loading subjects..." }
        },
        Some(Err(e)) => rsx! {
            div { 
                class: "main-container", 
                h2 { "Error" }
                p { "{e}" }
                button { onclick: move |_| topics_resource.restart(), "Retry" }
            }
        }
    }
}

#[component]
pub fn RandTopic(
    mut topic: Signal<RandObject>, 
    mut is_animating: Signal<bool>,
    id: i64,
    available_topics: Vec<RandObject>
) -> Element {
    let on_click = move |_| {
        if is_animating() {
            is_animating.set(false);
            return;
        }
        
        is_animating.set(true);
        let topics = available_topics.clone();
        
        spawn(async move {
            let timeout = tokio::time::sleep(Duration::from_secs(3));
            tokio::pin!(timeout);

            while is_animating() {
                tokio::select! {
                    _ = &mut timeout => {
                        is_animating.set(false);
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_millis(50)) => {
                        topic.set(topics
                            .choose(&mut rand::rng())
                            .cloned()
                            .unwrap_or_default()
                        );
                    }
                }
            }
        });
    };
    
    let on_reset = move |_| {
        topic.set(RandObject::new());
        is_animating.set(false);
    };

    let reset_class = if id != 0 && !is_animating() { "reset-button" } else { "reset-button hidden" };
    let rand_btn_class = if is_animating() { "rand-button animating" } else { "rand-button" };

    rsx! {
        div { 
            class: "button-container",
            button { 
                class: "{rand_btn_class}",
                onclick: on_click,
                if is_animating() {"STOP"} else {"RANDOMIZE"}
            }
            button { 
                class: "{reset_class}",
                onclick: on_reset,
                "RESET"
            }
        }
    }
}
