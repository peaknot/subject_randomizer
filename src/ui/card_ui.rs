use std::time::Duration;

use dioxus::{prelude::*};
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use rusqlite::Connection;


use crate::handles::randomize::query_topic;
use crate::handles::*;
use crate::application::*;
use crate::structs::AppState;

#[component]
pub fn Card() -> Element {

    let rand_topic = use_signal(|| String::from("Welcome"));

    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css")}
        div { 
            class: "header",
            "Subject Randomizer"
        }
        div {  
            class: "circle-container",
            div { 
                class: "circle",
                "{rand_topic}"
            }
        }
        RandTopic { topic: rand_topic }
    }
}

#[component]
pub fn RandTopic(mut topic: Signal<String>) -> Element {
    let db: AppState = use_context();
    let mut is_animating = use_signal(|| false);

    let on_click = move |_| {
        if is_animating() {
            is_animating.set(false);
        } else {
            is_animating.set(true);    
        }
        let db = db.clone();
        
        //let topics = query_topic(&conn).unwrap();
        spawn(async move {
            let timeout = tokio::time::sleep(Duration::from_secs(3));
            tokio::pin!(timeout);

            while is_animating() {
                tokio::select! {
                    _ = &mut timeout => {
                        is_animating.set(false);
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_millis(30)) => {
                        let conn = db.pool.lock().unwrap();
                        topic.set(rand_topic(&conn).unwrap().get_name());
                        drop(conn);        
                    }
                }
            }

        });

    };
    rsx! {
        button { 
            class: "rand-button",
            onclick: on_click,
            if is_animating() {"STOP"} else {"RANDOMIZE"}
        }
    }
}