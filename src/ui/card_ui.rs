use std::time::Duration;

use dioxus::html::g::to;
use dioxus::{prelude::*};
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use rusqlite::Connection;
use tokio::task::spawn_blocking;


use crate::handles::randomize::query_obj;
use crate::handles::randomize::query_topic;
use crate::handles::*;
use crate::application::*;
use crate::structs::AppState;
use crate::structs::module::RandObject;

#[component]
pub fn Card() -> Element {
    let r_obj = use_signal(|| RandObject::new());
    let is_animating = use_signal(|| false);

    let RandObject { 
        topic_name, 
        mod_id, 
        subj_name
    } =  r_obj();
    
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
                    class: "circle",
                    div { 
                        class: "obj-data",
                        
                        if !is_animating() && mod_id == 0 {
                            p { id: "placeholder", "Press Randomize to begin!"}
                        } else {
                            if !is_animating() { p { id: "module", "Module: {mod_id}" }}
                        }

                        p { id: "topic", "{topic_name}"}

                        if !is_animating() && !subj_name.is_empty() {
                            p { id: "subject", "Subject: {subj_name}"}
                        }
                     }
                }
            }
            RandTopic { topic: r_obj, is_animating, id: mod_id }
        }
        
    }
}

#[component]
pub fn RandTopic(
    mut topic: Signal<RandObject>, 
    mut is_animating: Signal<bool>,
    id: i64) -> Element {
    let db: AppState = use_context();

    let on_click = move |_| {
        
        if is_animating() {
            is_animating.set(false);
            return;
        }
        is_animating.set(true);
        let db = db.clone();
        
        spawn( async move {
            let topics = tokio::task::spawn_blocking(move || {
                let conn = db.pool.lock().unwrap();
                query_obj(&conn).unwrap()
            }).await.unwrap();
            // let topics = tokio::task::spawn_blocking(move || {
            //     let conn = db.pool.lock().unwrap();
            //     query_topic(&conn).unwrap()
            // }).await.unwrap();

            let timeout = tokio::time::sleep(Duration::from_secs(3));
            tokio::pin!(timeout);

            while is_animating() {
                tokio::select! {
                    _ = &mut timeout => {
                        is_animating.set(false);
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_nanos(100)) => {
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

    rsx! {
        div { 
            class: "button-container",
            button { 
                class: "rand-button",
                onclick: on_click,
                if is_animating() {"STOP"} else {"RANDOMIZE"}
            }
            if id != 0 && !is_animating() {
                button { 
                    class: "reset-button",
                    onclick: on_reset,
                    "RESET"
                }
            }
            
        }
    }
}