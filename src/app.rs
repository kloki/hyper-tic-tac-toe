#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style { include_str!("../src/style.css") }
            header { span{ class:"highlight","HYPER"} " tic-tac-toe" }
            div { class: "content", div {class: "board", "XO" } }
        }
    })
}
