#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::game::{
    Board,
    Pos,
};

pub fn App(cx: Scope) -> Element {
    let board = use_ref(cx, || Board::new());
    cx.render(rsx! {
        div {
            style { include_str!("../src/style.css") }
            header {
                span { class: "highlight", "HYPER" }
                " tic-tac-toe"
            }
            div { class: "content",
                div { class: "board",
                    Tile { pos: Pos::LU, board: board }
                    Tile { pos: Pos::MU, board: board }
                    Tile { pos: Pos::RU, board: board }
                    Tile { pos: Pos::LM, board: board }
                    Tile { pos: Pos::MM, board: board }
                    Tile { pos: Pos::RM, board: board }
                    Tile { pos: Pos::LD, board: board }
                    Tile { pos: Pos::MD, board: board }
                    Tile { pos: Pos::RD, board: board }
                }
            }
        }
    })
}

#[inline_props]
fn Tile<'a>(cx: Scope, pos: Pos, board: &'a UseRef<Board>) -> Element {
    let b = board.read();
    let mut border_class = "";

    if b.resolved(*pos) {
        let winner = b.boards[pos.index()].winner;
        return cx.render(rsx! {div { class: "sub-board resolved {pos.class()} {winner.class()}", "{winner.value()}" }});
    }

    if b.clickable_board(*pos) {
        border_class = b.current.class()
    }
    cx.render(rsx! {
        div { class: "sub-board {pos.class()} {border_class}",
            SubTile { pos1: *pos, pos2: Pos::LU, board: board }
            SubTile { pos1: *pos, pos2: Pos::MU, board: board }
            SubTile { pos1: *pos, pos2: Pos::RU, board: board }
            SubTile { pos1: *pos, pos2: Pos::LM, board: board }
            SubTile { pos1: *pos, pos2: Pos::MM, board: board }
            SubTile { pos1: *pos, pos2: Pos::RM, board: board }
            SubTile { pos1: *pos, pos2: Pos::LD, board: board }
            SubTile { pos1: *pos, pos2: Pos::MD, board: board }
            SubTile { pos1: *pos, pos2: Pos::RD, board: board }
        }
    })
}

#[inline_props]
fn SubTile<'a>(cx: Scope, pos1: Pos, pos2: Pos, board: &'a UseRef<Board>) -> Element {
    let b = board.read();
    let state = b.get(*pos1, *pos2);
    if b.clickable(*pos1, *pos2) {
        return cx.render(rsx! {
            div {
                class: "sub {pos2.class()} clickable",
                onclick: move |_| board.write().set(*pos1, *pos2),
                "{state.value()}"
            }
        });
    }

    return cx
        .render(rsx! { div { class: "sub {pos2.class()} {state.class()}", "{state.value()}" } });
}
