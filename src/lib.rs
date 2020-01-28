#![feature(track_caller)]

use comp_state::{topo, use_state, CloneState};
use seed::{prelude::*, *};

#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NoOp => (),
    }
}

fn view(_model: &Model) -> impl View<Msg> {
    root_view()
}

#[topo::nested]
fn root_view() -> Node<Msg> {
    div![
        "Clone Example:",
        div![
            my_button(),
            my_button(),
            my_button(),
            my_button(),
            my_button(),
        ],
        "None Clone:",
        div![
            my_button_non_clone(),
            my_button_non_clone(),
            my_button_non_clone(),
            my_button_non_clone(),
            my_button_non_clone(),
        ],
    ]
}

#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 3);
    div![
        button![
            "-",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v -= 1);
                Msg::NoOp
            }),
        ],
        count.get().to_string(),
        button![
            "+",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v += 1);
                Msg::NoOp
            }),
        ],
    ]
}

#[derive(Default)]
struct NonCloneI32(i32);

#[topo::nested]
fn my_button_non_clone() -> Node<Msg> {
    let count = use_state(NonCloneI32::default);

    div![
        button![
            "-",
            mouse_ev(Ev::Click, move |_| {
                count.update(|item| item.0 -= 1);
                Msg::NoOp
            }),
        ],
        count.get_with(|item| item.0.to_string()),
        button![
            "+",
            mouse_ev(Ev::Click, move |_| {
                count.update(|item| item.0 += 1);
                Msg::NoOp
            }),
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
