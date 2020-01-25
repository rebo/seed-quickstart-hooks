#![feature(track_caller)]

use comp_state::{topo, use_state};
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
        my_button(),
        my_button(),
        my_button(),
        my_button(),
        my_button(),
    ]
}

#[topo::nested]
fn my_button() -> Node<Msg> {
    let (count, count_access) = use_state(|| 0);
    div![button![
        format!("Clicked {} times", count),
        mouse_ev(Ev::Click, move |_| {
            count_access.set(count + 1);
            Msg::NoOp
        })
    ]]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
