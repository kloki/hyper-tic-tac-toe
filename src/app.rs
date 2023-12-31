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
            div { class: "banner", onclick: move |_| board.set(Board::new()),
                span { class: "highlight", "HYPER" }
                " tic-tac-toe"
            }
            div { class: "center",
                div { class: "content",
                    Message { board: board }
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
                    div { class: "bottom",
                        a { href: "https://github.com/kloki/hyper-tic-tac-toe", "code" }
                        a { href: "https://en.wikipedia.org/wiki/Ultimate_tic-tac-toe",
                            "wiki"
                        }
                        button { onclick: move |_| board.set(Board::new()), "reset" }
                    }
                }
            }
        }
    })
}

#[inline_props]
fn Message<'a>(cx: Scope, board: &'a UseRef<Board>) -> Element {
    let b = board.read();
    if b.winner.resolved() {
        return cx.render(rsx! {
            div { class: "message",
                "Player "
                span { class: "{b.winner.class()}", "{b.winner.value()}" }
                " is the winner!!!"
            }
        });
    }

    if b.tied() {
        return cx.render(rsx! {
            div { class: "message", div { "No possibbe moves. Its a tie!" } }
        });
    }

    cx.render(rsx! {
        div { class: "message",
            div {
                "Player "
                span { class: "{b.current.class()}", "{b.current.value()}" }
                "'s turn"
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
