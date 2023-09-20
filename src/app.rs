#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq)]
pub enum Pos {
    LU,
    MU,
    RU,
    LM,
    MM,
    RM,
    LD,
    MD,
    RD,
}

impl Pos {
    fn class(&self) -> &str {
        match self {
            Pos::LU => "lu",
            Pos::MU => "mu",
            Pos::RU => "ru",
            Pos::LM => "lm",
            Pos::MM => "mm",
            Pos::RM => "rm",
            Pos::LD => "ld",
            Pos::MD => "md",
            Pos::RD => "rd",
        }
    }
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style { include_str!("../src/style.css") }
            header { span{ class:"highlight","HYPER"} " tic-tac-toe" }
            div { class: "content", div {class: "board",
                Tile{pos: Pos::LU}
                Tile{pos: Pos::MU}
                Tile{pos: Pos::RU}
                Tile{pos: Pos::LM}
                Tile{pos: Pos::MM}
                Tile{pos: Pos::RM}
                Tile{pos: Pos::LD}
                Tile{pos: Pos::MD}
                Tile{pos: Pos::RD}
            } }
        }
    })
}

#[inline_props]
fn Tile(cx: Scope, pos: Pos) -> Element {
    cx.render(rsx! {
            div{ class:"sub-board {pos.class()}",
                SubTile{pos: Pos::LU}
                SubTile{pos: Pos::MU}
                SubTile{pos: Pos::RU}
                SubTile{pos: Pos::LM}
                SubTile{pos: Pos::MM}
                SubTile{pos: Pos::RM}
                SubTile{pos: Pos::LD}
                SubTile{pos: Pos::MD}
                SubTile{pos: Pos::RD}
            }
    })
}

#[inline_props]
fn SubTile(cx: Scope, pos: Pos) -> Element {
    cx.render(rsx! {div {class:"sub {pos.class()}" , "X" }})
}
